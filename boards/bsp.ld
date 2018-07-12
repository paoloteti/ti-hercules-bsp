
MEMORY
{
    rom (rx)     : ORIGIN = ROM_ORIGIN,   LENGTH = ROM_LENGTH
    stack (rw)   : ORIGIN = STACK_ORIGIN, LENGTH = STACK_LENGTH
    ram (rwx)    : ORIGIN = RAM_ORIGIN,   LENGTH = RAM_LENGTH
    vimram (rx)  : ORIGIN = VIM_ORIGIN,   LENGTH = VIM_LENGTH
}

__stack_size__ = DEFINED(STACK_LENGTH);

SECTIONS
{
    /* ARM Cortex-R4 Vector Table */
    .intvecs :
    {
        . = ALIGN(4);
        KEEP(*(.intvecs))
    } > rom

    .text :
    {
        . = ALIGN(4);
            _stext = .;

        /*
            * .gnu.linkonce hold C++ elements with vague linkage
            *		https://gcc.gnu.org/onlinedocs/gcc/Vague-Linkage.html
            */
        *(.text .text.* .gnu.linkonce.t.*)
        *(.rodata .rodata* .gnu.linkonce.r.*)

        /* C++ exception unwinding information */
        *(.ARM.extab* .gnu.linkonce.armextab.*)

        /*
            * glue_7 and glue_7t hold helper functions emitted by the compiler to
            * support interworking (linking between functions in ARM and THUMB
            *  mode). Note that Cortex-M's do not support ARM mode, but this is left
            *  here to save someone headache if they ever attempt to port Tock to a
            *  Cortex-A core.
            */
        *(.glue_7t) *(.glue_7)


        /*
            * Constructor and destructor sections:
            *
            *  - init/fini
            *	  Defined by ELF as sections that hold `process
            *	  initialization/termination code`
            *  - {pre}{init/fini}_array_{start/end}
            *	  Symbols used by the C runtime for initialization / termination
            *  - ctors/dtors
            *	  Symbols used by the C++ runtime for initialization / termination
            */
        . = ALIGN(4);
        KEEP(*(.init))
        . = ALIGN(4);
        __preinit_array_start = .;
        KEEP (*(.preinit_array))
        __preinit_array_end = .;

        . = ALIGN(4);
        __init_array_start = .;
        KEEP (*(SORT(.init_array.*)))
        KEEP (*(.init_array))
        __init_array_end = .;

        . = ALIGN(4);
        KEEP (*crtbegin.o(.ctors))
        KEEP (*(EXCLUDE_FILE (*crtend.o) .ctors))
        KEEP (*(SORT(.ctors.*)))
        KEEP (*crtend.o(.ctors))

        . = ALIGN(4);
        KEEP(*(.fini))

        . = ALIGN(4);
        __fini_array_start = .;
        KEEP (*(.fini_array))
        KEEP (*(SORT(.fini_array.*)))
        __fini_array_end = .;

        KEEP (*crtbegin.o(.dtors))
        KEEP (*(EXCLUDE_FILE (*crtend.o) .dtors))
        KEEP (*(SORT(.dtors.*)))
        KEEP (*crtend.o(.dtors))
    } > rom

    /* ARM Exception support
        *
        * This contains compiler-generated support for unwinding the stack,
        * consisting of key-value pairs of function addresses and information on
        * how to unwind stack frames.
        * https://wiki.linaro.org/KenWerner/Sandbox/libunwind?action=AttachFile&do=get&target=libunwind-LDS.pdf
        *
        * .ARM.exidx is sorted, so has to go in its own output section.
        */
    PROVIDE_HIDDEN (__exidx_start = .);
    .ARM.exidx :
    {
        /* (C++) Index entries for section unwinding */
        *(.ARM.exidx* .gnu.linkonce.armexidx.*)
    } > rom
    PROVIDE_HIDDEN (__exidx_end = .);

    . = ALIGN(4);
    _etext = .;

    .data :
    {
        _sidata = LOADADDR(.data);
        . = ALIGN(4);
        _sdata = .;
        *(.data .data.*);
        . = ALIGN(4);
        _edata = .;
    } > ram AT > rom

    .stack (NOLOAD) :
    {
        . = ALIGN(8);
            _sstack = .;
        . = . + __stack_size__;
        . = ALIGN(8);
        _estack = .;
    } > stack

    .sram (NOLOAD) :
    {
        /* BSS section. */
        . = ALIGN(4);
        _sbss = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
        . = ALIGN(MPU_MIN_ALIGN);
    } > ram

    .vim (NOLOAD) :
    {
        . = ALIGN(4);
            KEEP(*(.vim_table .vim_table.*))
        . = ALIGN(4);
    } > vimram
}
