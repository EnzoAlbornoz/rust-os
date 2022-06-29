/**********************************************
 * Define where the memory and program begins *
 **********************************************/

__arm64_phy_dram_start_addr__ = 0x00000;
__arm64_stacks_start_addr__   = 0x01800;
__arm64_phy_prog_start_addr__ = 0x80000;

/**********************************************/

/* *********************************************
 * Global variables to expose to the program  *
 **********************************************/

/********************************************* */

/**********************************************
 * Tell the linker where is the program entrypoint
 **********************************************/

ENTRY(__arm64_phy_prog_start_addr__);

/**********************************************/

/**********************************************
 * Define metadata (program headers)
 * about our regions
 * 
 * Permissions - R = 4; RX = 5; RW = 6;
 * Using PT_LOAD to provide Phy and Log Addresses
 **********************************************/

PHDRS {
    exception_handlers  PT_LOAD FLAGS(6);
    boot_stacks         PT_LOAD FLAGS(6);
    kernel_code         PT_LOAD FLAGS(5);
    kernel_data         PT_LOAD FLAGS(6);
}

/**********************************************/

/**********************************************
 * Define program memory layout
 **********************************************/

SECTIONS {
    /* Initialize linker cursor on DRAM start */
    . = __arm64_phy_dram_start_addr__;
    /******************************************
     * Vector Tables                          *
     *----------------------------------------*
     * In AArch64, we do not have a simple    *
     * array as a Exception Handler pointer   * 
     * list, but a hole table cointaining the *
     * handlers (code). In order to allocate  *
     * it we assign a section to it.          *
     ******************************************/
    .kernel_vector_table : ALIGN(2048) {
        __vector_tables_start__ = .;
        KEEP(*(SORT_BY_NAME(.kernel_vector_table*)))
        __vector_tables_end__ = .;
    } :exception_handlers
    /******************************************
     * Cores Boot Stacks                      *
     *----------------------------------------*
     * As the code starts in a memory region  *
     * different from DRAM start, we use this *
     * empty space to allocate the cores boot *
     * stacks.                                *
     ******************************************/
    .core_boot_stacks (NOLOAD): ALIGN(16) {
        __boot_stacks_start__ = .;
        __boot_stacks_end__ = ADDR(.text);
    } :boot_stacks

    /* Advance cursor to the definition of the program */
    . = __arm64_phy_prog_start_addr__;
    /******************************************
     * Kernel Code (Instructions + Read Only) *
     ******************************************/
    .text : {
        KEEP(*(.text._start)) /* System prelude */
        *(.text._start_arguments) /* Constants (or statics in Rust speak) read by _start(). */
        *(.text._start_rust) /* Rust Kernel entrypoint */
        *(.text*) /* Everything else */
    } :kernel_code

    /* Read Only Data */
    .rodata : ALIGN(8) {
        *(.rodata*)
    } :kernel_code

    /* Global Variables */
    .got : ALIGN(8) {
        *(.got*)
    } :kernel_code

    /******************************************
     * Kernel Data (Data + BSS)               *
     ******************************************/
    .data : {
        __data_start__ = .;
        *(.data*)
        __data_end__ = .;
    } :kernel_data

    /* 16 bytes aligned of 64 bits values that need to be initialized with zero */
    .bss (NOLOAD): ALIGN(16) {
        __bss_start__ = .;
        *(.bss*);
        __bss_end__ = .;
    } :kernel_data
}