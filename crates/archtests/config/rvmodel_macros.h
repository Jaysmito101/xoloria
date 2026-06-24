#ifndef _RVMODEL_MACROS_H
#define _RVMODEL_MACROS_H


#define STANDARD_SM_SUPPORTED

##### STARTUP #####

# Perform boot operations. Can be empty or left undefined unless needed for
# DUT-specific behavior such as turning on a memory controller or
# initializing custom state.
//#define RVMODEL_BOOT

// Custom RVMODEL_BOOT_TO_MMODE overrides default RVTEST_BOOT_TO_MMODE
// if defined.  For most DUTs, the default should work and this macro
// should not be defined.  If no M-mode or CSRs are implemented, define this
// macro as blank to bypass the boot process.  If a nonconforming
// M-mode is implemented, define this macro to set up the necessary
// state in a fashion similar to RVTEST_BOOT_TO_MMODE.
#define RVMODEL_BOOT_TO_MMODE

##### TERMINATION #####

#define XOLORIA_HOST_EXIT_ADDRESS 0x1000
#define XOLORIA_HOST_EXIT_PASS 0x1
#define XOLORIA_HOST_EXIT_FAIL 0x2

# Terminate test with a pass indication.
# When the test is run in simulation, this should end the simulation.
#define RVMODEL_HALT_PASS  \
  li x1, XOLORIA_HOST_EXIT_PASS        ;\
  la t0, XOLORIA_HOST_EXIT_ADDRESS     ;\
  sw x1, 0(t0)                         ;\
  self_loop_pass:                      ;\
    j self_loop_pass                   ;\

# Terminate test with a fail indication.
# When the test is run in simulation, this should end the simulation.
#define RVMODEL_HALT_FAIL \
  li x1, XOLORIA_HOST_EXIT_FAIL        ;\
  la t0, XOLORIA_HOST_EXIT_ADDRESS     ;\
  sw x1, 0(t0)                         ;\
  self_loop_fail:                      ;\
    j self_loop_fail      ;\


#endif // _RVMODEL_MACROS_H
