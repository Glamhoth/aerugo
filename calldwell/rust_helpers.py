"""Module containing some boilerplate, common to all tests that use `calldwell-rs` library."""
import logging
from typing import Any, Callable, Optional, Tuple

from .gdb_client import GDBClient
from .rtt_client import CalldwellRTTClient

RTT_SECTION_SYMBOL_NAME = "_SEGGER_RTT"
"""Section name of RTT symbol. Hard-coded in `rtt_target` library."""
RTT_SECTION_SEARCHED_MEMORY_LENGTH = 0x800
"""This constant defines the amount of bytes OpenOCD will search, looking for RTT section.
Current value might as well be an overkill, but it works."""
RTT_SECTION_ID = "SEGGER RTT"
"""RTT section ID"""
CALLDWELL_INIT_FUNCTION_NAME = "calldwell::initialize"
"""Name of Calldwell-rs initialization function. Note that this is for `debug` binary,
as mangled `release` names are not supported (yet)."""

EXPECTED_MCU_INIT_MESSAGE = "calldwell-rs started"
"""Message that MCU should send when it finishes RTT initialization"""
HOST_HANDSHAKE_MESSAGE = "host handshake requested"
"""Response to MCU init message, request for handshake"""
EXPECTED_MCU_HANDSHAKE_MESSAGE = f"{len(HOST_HANDSHAKE_MESSAGE)}:{HOST_HANDSHAKE_MESSAGE}"
"""Expected MCU response to handshake request"""


def init_remote_calldwell_rs_session(
    gdb_executable: str,
    gdb_server_hostname: str,
    gdb_server_port: int,
    rtt_server_port: int,
    path_to_test_executable: str,
    gdb_timeout: Optional[float] = None,
    log_responses: bool = False,
    log_execution: bool = False,
    pre_handshake_hook: Optional[Callable[[GDBClient, Optional[Any]], None]] = None,
    pre_handshake_hook_argument: Optional[Any] = None,
) -> Optional[Tuple[GDBClient, CalldwellRTTClient]]:
    """Initializes Calldwell-rs test session by connecting to GDB server (like OpenOCD),
    starting RTT server, flashing the executable, waiting until `calldwell::initialize`
    executes, and performing handshake (and optional pre-handshake hook, if provided).

    This function returns a tuple containing `GDBClient` with program in stopped state right after
    `calldwell:initialize` execution, and `RTTClient`, or `None` if starting the session fails at
    any point.

    Provided test executable is in running state after this function finishes.

    This function can also throw one of the `pygdbmi` exceptions, like `GdbTimeoutError`.

    To check what caused the issue, you should check the logs. Writing proper error handling
    for all the scenarios is certainly possible, but usually the error is unrecoverable and
    should fail the test anyway, so there's no point in proper error handling anyway.

    Optional pre-handshake hook will be executed while program is stopped.
    Pre-handshake hook is given `GDBClient` instance as first argument, and user-provided
    `pre_handshake_hook_argument` as second.

    # Parameters
    * `gdb_executable` - Path to GDB executable
    * `gdb_server_hostname` - Network path to GDB server. If ran locally, use `localhost`
    * `gdb_server_port` - Network port of GDB server
    * `rtt_server_port` - Port for RTT communication that will be opened by GDB
    * `path_to_test_executable` - Path to Calldwell test executable
    * `gdb_timeout` - Timeout for GDBClient, if `None` then default one will be used.
    * `log_responses` - Whether to log GDB/MI responses, or not
    * `log_execution` - Whether to log the execution of GDB commands, or not
    * `pre_handshake_hook` - Function that will be called before performing Calldwell handshake.
                             It can be used to perform some GDB operations before the program
                             starts normal execution.
    * `pre_handshake_hook_argument` - User argument passed to `pre_handshake_hook`, if present.
    """
    gdb_server_full_hostname = f"{gdb_server_hostname}:{gdb_server_port}"
    gdb = GDBClient(
        gdb_executable,
        gdb_timeout,
        log_responses,
        log_execution,
    )

    if not gdb.connect_to_remote(gdb_server_full_hostname):
        logging.error(f"Could not connect to remote GDB server @ {gdb_server_full_hostname}")
        return None

    if not gdb.start_rtt_server(rtt_server_port, 0):
        logging.error(f"Could not start RTT server @ TCP port {rtt_server_port}")
        return None

    rtt = CalldwellRTTClient(gdb_server_hostname, rtt_server_port)

    if not gdb.load_executable(path_to_test_executable):
        logging.error(f"Could not load executable {path_to_test_executable} into MCU memory")
        return None

    rtt_symbol = gdb.get_variable(RTT_SECTION_SYMBOL_NAME)
    if rtt_symbol is None:
        logging.error(f"Could not find symbol for RTT section {RTT_SECTION_SYMBOL_NAME}")
        return None

    if not gdb.start_program():
        logging.error("Could not start execution of test program")
        return None

    if not gdb.setup_rtt(rtt_symbol.address, RTT_SECTION_SEARCHED_MEMORY_LENGTH, RTT_SECTION_ID):
        logging.error(
            f"Could not setup RTT for section @ {rtt_symbol.address} "
            "(searched {RTT_SECTION_SEARCHED_MEMORY_LENGTH} bytes)"
        )
        return None

    if not gdb.set_breakpoint(CALLDWELL_INIT_FUNCTION_NAME):
        logging.error(f"Could not set breakpoint @ {CALLDWELL_INIT_FUNCTION_NAME}")
        return None

    gdb.continue_program()

    if not gdb.wait_for_breakpoint_hit():
        logging.error("Program has stopped, but not because of a breakpoint")
        return None

    gdb.finish_function_execution()

    if not gdb.start_rtt():
        logging.error("Could not start RTT (probably because the section wasn't found)")
        return None

    if pre_handshake_hook is not None:
        pre_handshake_hook(gdb, pre_handshake_hook_argument)

    gdb.continue_program()

    if not _perform_handshake(rtt):
        logging.error("Couldn't perform correct handshake with MCU")
        return None

    return gdb, rtt


def _perform_handshake(rtt: CalldwellRTTClient) -> bool:
    """Performs Calldwell handshake after it's RTT facilities are started.
    This acts like a mini self-test of RTT communication, to guarantee that it works correctly.
    """
    init_message = rtt.receive_string_stream()

    if init_message != EXPECTED_MCU_INIT_MESSAGE:
        logging.error(
            "Received unexpected MCU init message "
            f"(got '{init_message}', expected '{EXPECTED_MCU_INIT_MESSAGE}')"
        )
        return False

    rtt.transmit_string_stream(HOST_HANDSHAKE_MESSAGE)
    response = rtt.receive_string_stream()

    if response != EXPECTED_MCU_HANDSHAKE_MESSAGE:
        logging.error(
            "MCU responded with invalid handshake message "
            f"(got '{response}', expected '{EXPECTED_MCU_HANDSHAKE_MESSAGE}')"
        )
        return False

    return True