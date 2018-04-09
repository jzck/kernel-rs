extern crate raw_cpuid;

use core::fmt::Result;

use self::raw_cpuid::CpuId;

pub fn cpu_info() -> Result {
    let cpuid = CpuId::new();

    if let Some(info) = cpuid.get_vendor_info() {
        println!("Vendor: {}", info.as_string());
    }

    if let Some(info) = cpuid.get_extended_function_info() {
        if let Some(brand) = info.processor_brand_string() {
            println!("Model: {}", brand);
        }
    }

    if let Some(info) = cpuid.get_processor_frequency_info() {
        println!("Base MHz: {}", info.processor_base_frequency());
        println!("Max MHz: {}", info.processor_max_frequency());
        println!("Bus MHz: {}", info.bus_frequency());
    } else {
        set_color!(Red);
        println!("Couldn't retrieve cpu frequency info");
        set_color!();
    }

    if let Some(info) = cpuid.get_feature_info() {
        print!("Features:");
        if info.has_fpu() {
            print!(" fpu")
        };
        if info.has_vme() {
            print!(", vme")
        };
        if info.has_de() {
            print!(", de")
        };
        if info.has_pse() {
            print!(", pse")
        };
        if info.has_tsc() {
            print!(", tsc")
        };
        if info.has_msr() {
            print!(", msr")
        };
        if info.has_pae() {
            print!(", pae")
        };
        if info.has_mce() {
            print!(", mce")
        };

        if info.has_cmpxchg8b() {
            print!(", cx8")
        };
        if info.has_apic() {
            print!(", apic")
        };
        if info.has_sysenter_sysexit() {
            print!(", sep")
        };
        if info.has_mtrr() {
            print!(", mtrr")
        };
        if info.has_pge() {
            print!(", pge")
        };
        if info.has_mca() {
            print!(", mca")
        };
        if info.has_cmov() {
            print!(", cmov")
        };
        if info.has_pat() {
            print!(", pat")
        };

        if info.has_pse36() {
            print!(", pse36")
        };
        if info.has_psn() {
            print!(", psn")
        };
        if info.has_clflush() {
            print!(", clflush")
        };
        if info.has_ds() {
            print!(", ds")
        };
        if info.has_acpi() {
            print!(", acpi")
        };
        if info.has_mmx() {
            print!(", mmx")
        };
        if info.has_fxsave_fxstor() {
            print!(", fxsr")
        };
        if info.has_sse() {
            print!(", sse")
        };

        if info.has_sse2() {
            print!(", sse2")
        };
        if info.has_ss() {
            print!(", ss")
        };
        if info.has_htt() {
            print!(", ht")
        };
        if info.has_tm() {
            print!(", tm")
        };
        if info.has_pbe() {
            print!(", pbe")
        };

        if info.has_sse3() {
            print!(", sse3")
        };
        if info.has_pclmulqdq() {
            print!(", pclmulqdq")
        };
        if info.has_ds_area() {
            print!(", dtes64")
        };
        if info.has_monitor_mwait() {
            print!(", monitor")
        };
        if info.has_cpl() {
            print!(", ds_cpl")
        };
        if info.has_vmx() {
            print!(", vmx")
        };
        if info.has_smx() {
            print!(", smx")
        };
        if info.has_eist() {
            print!(", est")
        };

        if info.has_tm2() {
            print!(", tm2")
        };
        if info.has_ssse3() {
            print!(", ssse3")
        };
        if info.has_cnxtid() {
            print!(", cnxtid")
        };
        if info.has_fma() {
            print!(", fma")
        };
        if info.has_cmpxchg16b() {
            print!(", cx16")
        };
        if info.has_pdcm() {
            print!(", pdcm")
        };
        if info.has_pcid() {
            print!(", pcid")
        };
        if info.has_dca() {
            print!(", dca")
        };

        if info.has_sse41() {
            print!(", sse4_1")
        };
        if info.has_sse42() {
            print!(", sse4_2")
        };
        if info.has_x2apic() {
            print!(", x2apic")
        };
        if info.has_movbe() {
            print!(", movbe")
        };
        if info.has_popcnt() {
            print!(", popcnt")
        };
        if info.has_tsc_deadline() {
            print!(", tsc_deadline_timer")
        };
        if info.has_aesni() {
            print!(", aes")
        };
        if info.has_xsave() {
            print!(", xsave")
        };

        if info.has_oxsave() {
            print!(", xsaveopt")
        };
        if info.has_avx() {
            print!(", avx")
        };
        if info.has_f16c() {
            print!(", f16c")
        };
        if info.has_rdrand() {
            print!(", rdrand")
        };
        println!("");
    }

    if let Some(info) = cpuid.get_extended_function_info() {
        print!("Extended function:");
        if info.has_64bit_mode() {
            print!(" lm")
        };
        if info.has_rdtscp() {
            print!(", rdtscp")
        };
        if info.has_1gib_pages() {
            print!(", pdpe1gb")
        };
        if info.has_execute_disable() {
            print!(", nx")
        };
        if info.has_syscall_sysret() {
            print!(", syscall")
        };
        if info.has_prefetchw() {
            print!(", prefetchw")
        };
        if info.has_lzcnt() {
            print!(", lzcnt")
        };
        if info.has_lahf_sahf() {
            print!(", lahf_lm")
        };
        if info.has_invariant_tsc() {
            print!(", constant_tsc")
        };
        println!("");
    }

    if let Some(info) = cpuid.get_extended_feature_info() {
        print!("Extended features:");
        if info.has_fsgsbase() {
            print!(" fsgsbase")
        };
        if info.has_tsc_adjust_msr() {
            print!(", tsc_adjust")
        };
        if info.has_bmi1() {
            print!(", bmi1")
        };
        if info.has_hle() {
            print!(", hle")
        };
        if info.has_avx2() {
            print!(", avx2")
        };
        if info.has_smep() {
            print!(", smep")
        };
        if info.has_bmi2() {
            print!(", bmi2")
        };
        if info.has_rep_movsb_stosb() {
            print!(", erms")
        };
        if info.has_invpcid() {
            print!(", invpcid")
        };
        if info.has_rtm() {
            print!(", rtm")
        };
        if info.has_qm() {
            print!(", qm")
        };
        if info.has_fpu_cs_ds_deprecated() {
            print!(", fpu_seg")
        };
        if info.has_mpx() {
            print!(", mpx")
        };
        println!("");
    }

    Ok(())
}
