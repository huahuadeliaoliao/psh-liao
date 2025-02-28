// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of PSH.
//
// PSH is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// PSH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Performance Savior Home (PSH). If not,
// see <https://www.gnu.org/licenses/>.

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use super::{AddressSizes, Arm64CpuInfo, CpuInfo, TlbSize, X86_64CpuInfo};

fn parse_unit(unit: &str) -> u32 {
    match unit.trim() {
        "B" => 1,
        "KB" => 1024,
        "MB" => 1024 * 1024,
        _ => 0,
    }
}

fn parse_cache_size(value: &str) -> u32 {
    value
        .split_once(' ')
        .map(|(num, unit)| {
            let num: u32 = num.trim().parse().unwrap_or(0);
            let unit = parse_unit(unit);
            num * unit
        })
        .unwrap_or(0)
}

fn parse_address_sizes(value: &str) -> AddressSizes {
    let (phy, virt) = value
        .split_once(',')
        .map(|(phy, virt)| {
            let get_bits = |s: &str| {
                s.trim()
                    .split_ascii_whitespace()
                    .next()
                    .and_then(|a| a.parse().ok())
                    .unwrap_or(0u8)
            };
            (get_bits(phy), get_bits(virt))
        })
        .unwrap_or((0, 0));
    AddressSizes { phy, virt }
}

fn parse_tlb_size(value: &str) -> TlbSize {
    let (count, unit) = value
        .split_once(' ')
        .map(|(count, unit)| {
            let count = count.trim().parse().unwrap_or(0u32);
            let unit = if unit == "4K pages" { 4096 } else { 0 };
            (count, unit)
        })
        .unwrap_or((0, 0));
    TlbSize { count, unit }
}

fn parse_x86_64_cpu_info(reader: BufReader<File>) -> io::Result<Vec<X86_64CpuInfo>> {
    let mut cpu_info_list = Vec::new();
    let mut current_cpu_info = X86_64CpuInfo::new();

    for line in reader.lines().map_while(Result::ok) {
        if line.is_empty() {
            // Empty line indicates the end of one CPU's information
            cpu_info_list.push(current_cpu_info);
            current_cpu_info = X86_64CpuInfo::new();
            continue;
        }

        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "processor" => {
                    current_cpu_info.processor = value.parse().unwrap_or(0);
                }
                "vendor_id" => {
                    current_cpu_info.vendor_id = value.to_string();
                }
                "model name" => {
                    current_cpu_info.model_name = value.to_string();
                }
                "cpu family" => {
                    current_cpu_info.cpu_family = value.parse().unwrap_or(0);
                }
                "model" => {
                    current_cpu_info.model = value.parse().unwrap_or(0);
                }
                "stepping" => {
                    current_cpu_info.stepping = value.parse().unwrap_or(0);
                }
                "microcode" => {
                    current_cpu_info.microcode = value.to_string();
                }
                "cpu MHz" => {
                    current_cpu_info.cpu_mhz = value.parse().unwrap_or(0.0);
                }
                "cache size" => {
                    current_cpu_info.cache_size = parse_cache_size(value);
                }
                "physical id" => {
                    current_cpu_info.physical_id = value.parse().unwrap_or(0);
                }
                "siblings" => {
                    current_cpu_info.siblings = value.parse().unwrap_or(0);
                }
                "core id" => {
                    current_cpu_info.core_id = value.parse().unwrap_or(0);
                }
                "cpu cores" => {
                    current_cpu_info.cpu_cores = value.parse().unwrap_or(0);
                }
                "apicid" => {
                    current_cpu_info.apicid = value.parse().unwrap_or(0);
                }
                "initial apicid" => {
                    current_cpu_info.initial_apicid = value.parse().unwrap_or(0);
                }
                "fpu" => {
                    current_cpu_info.fpu = value == "yes";
                }
                "fpu_exception" => {
                    current_cpu_info.fpu_exception = value == "yes";
                }
                "cpuid level" => {
                    current_cpu_info.cpuid_level = value.parse().unwrap_or(0);
                }
                "wp" => {
                    current_cpu_info.wp = value == "yes";
                }
                "flags" => {
                    current_cpu_info.flags = value
                        .split_ascii_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                }
                "bugs" => {
                    current_cpu_info.bugs = value
                        .split_ascii_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                }
                "bogomips" => {
                    current_cpu_info.bogomips = value.parse().unwrap_or(0.0);
                }
                "TLB size" => {
                    current_cpu_info.tlb_size = parse_tlb_size(value);
                }
                "clflush size" => {
                    current_cpu_info.clflush_size = value.parse().unwrap_or(0);
                }
                "cache_alignment" => {
                    current_cpu_info.cache_alignment = value.parse().unwrap_or(0);
                }
                "address sizes" => {
                    current_cpu_info.address_sizes = parse_address_sizes(value);
                }
                "power management" => {
                    current_cpu_info.power_management = value
                        .split_ascii_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                }
                // Add other cases for fields you want to extract
                _ => {}
            }
        }
    }

    Ok(cpu_info_list)
}

fn parse_aarch64_cpu_info(reader: BufReader<File>) -> io::Result<Vec<Arm64CpuInfo>> {
    let mut cpu_info_list = Vec::new();
    let mut current_cpu_info = Arm64CpuInfo::new();

    for line in reader.lines().map_while(Result::ok) {
        if line.is_empty() {
            // Empty line indicates the end of one CPU's information
            cpu_info_list.push(current_cpu_info);
            current_cpu_info = Arm64CpuInfo::new();
            continue;
        }

        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "processor" => {
                    current_cpu_info.processor = value.parse().unwrap_or(0);
                }
                "BogoMIPS" => {
                    current_cpu_info.bogomips = value.parse().unwrap_or(0.0);
                }
                "Features" => {
                    let features = value.split(' ').map(|s| s.to_string()).collect();
                    current_cpu_info.features = features;
                }
                "CPU implementer" => {
                    current_cpu_info.cpu_implementer =
                        u16::from_str_radix(value.trim_start_matches("0x"), 16).unwrap_or(0);
                }
                "CPU architecture" => {
                    current_cpu_info.cpu_architecture = value.parse().unwrap_or(0);
                }
                "CPU variant" => {
                    current_cpu_info.cpu_variant =
                        u16::from_str_radix(value.trim_start_matches("0x"), 16).unwrap_or(0);
                }
                "CPU part" => {
                    current_cpu_info.cpu_part =
                        u16::from_str_radix(value.trim_start_matches("0x"), 16).unwrap_or(0);
                }
                "CPU revision" => {
                    current_cpu_info.cpu_revision = value.parse().unwrap_or(0);
                }
                "address sizes" => {
                    current_cpu_info.address_sizes = parse_address_sizes(value);
                }
                // Add other cases for fields you want to extract
                _ => {}
            }
        }
    }

    Ok(cpu_info_list)
}

#[allow(dead_code)]
pub fn do_parse_cpuinfo(path: &str, arch: &str) -> io::Result<CpuInfo> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let cpu_info = match arch {
        "x86_64" => {
            let x86_64_cpu_info = parse_x86_64_cpu_info(reader)?;
            CpuInfo::X86_64(x86_64_cpu_info)
        }
        "aarch64" => {
            let aarch64_cpu_info = parse_aarch64_cpu_info(reader)?;
            CpuInfo::Arm64(aarch64_cpu_info)
        }
        _ => CpuInfo::Unsupported(format!("unsupported architecture {}", arch)),
    };

    Ok(cpu_info)
}

#[allow(unused_macros)]
macro_rules! parse_cpuinfo {
    ($path:expr, $arch:expr) => {
        crate::cpu::raw::do_parse_cpuinfo($path, $arch)
    };
    () => {
        crate::cpu::raw::do_parse_cpuinfo("/proc/cpuinfo", &std::env::consts::ARCH)
    };
}

pub(crate) use parse_cpuinfo;

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::cpu::{AddressSizes, Arm64CpuInfo, CpuInfo, TlbSize, X86_64CpuInfo};

    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_cpu_info() {
        let cpus = num_cpus::get();
        let cpu_info = parse_cpuinfo!().unwrap();
        match cpu_info {
            CpuInfo::X86_64(x86_64_cpu_info) => {
                assert_eq!(cpus, x86_64_cpu_info.len());
            }
            CpuInfo::Arm64(aarch64_cpu_info) => {
                assert_eq!(cpus, aarch64_cpu_info.len());
            }
            _ => {
                panic!("Unknown CPU architecture");
            }
        }
    }

    #[test]
    fn test_parse_cpu_info_yitian() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("./test_resources/arch/aarch64/t-head/cpuinfo");
        let binding = d.into_os_string();
        let cpuinfo_path = binding.to_str().unwrap();

        let cpus = 128;
        let cpu_info = parse_cpuinfo!(cpuinfo_path, "aarch64").unwrap();
        match cpu_info {
            CpuInfo::X86_64(_) => {
                panic!("Should not reach here");
            }
            CpuInfo::Arm64(cpu_vec) => {
                let cpu126 = Arm64CpuInfo {
                    processor: 126,
                    bogomips: 100.0,
                    features: vec![
                        "fp",
                        "asimd",
                        "evtstrm",
                        "aes",
                        "pmull",
                        "sha1",
                        "sha2",
                        "crc32",
                        "atomics",
                        "fphp",
                        "asimdhp",
                        "cpuid",
                        "asimdrdm",
                        "jscvt",
                        "fcma",
                        "lrcpc",
                        "dcpop",
                        "sha3",
                        "sm3",
                        "sm4",
                        "asimddp",
                        "sha512",
                        "sve",
                        "asimdfhm",
                        "dit",
                        "uscat",
                        "ilrcpc",
                        "flagm",
                        "ssbs",
                        "sb",
                        "dcpodp",
                        "sve2",
                        "sveaes",
                        "svepmull",
                        "svebitperm",
                        "svesha3",
                        "svesm4",
                        "flagm2",
                        "frint",
                        "svei8mm",
                        "svebf16",
                        "i8mm",
                        "bf16",
                        "dgh",
                    ]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                    cpu_implementer: 65,
                    cpu_architecture: 8,
                    cpu_variant: 0,
                    cpu_part: 3401,
                    cpu_revision: 0,
                    address_sizes: AddressSizes { phy: 48, virt: 48 },
                };
                assert_eq!(cpu126, cpu_vec[126]);
                assert_eq!(cpus, cpu_vec.len());
            }
            _ => {
                panic!("Unknown CPU architecture");
            }
        }
    }

    #[test]
    fn test_parse_cpu_info_intel() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("./test_resources/arch/x86_64/intel/cpuinfo");
        let binding = d.into_os_string();
        let cpuinfo_path = binding.to_str().unwrap();

        let cpus = 2;
        let cpu_info = parse_cpuinfo!(cpuinfo_path, "x86_64").unwrap();
        match cpu_info {
            CpuInfo::X86_64(cpu_vec) => {
                let cpu0 = X86_64CpuInfo {
                    processor: 0,
                    vendor_id: "GenuineIntel".to_string(),
                    model_name: "Intel(R) Xeon(R) Platinum 8269CY CPU @ 2.50GHz".to_string(),
                    cpu_family: 6,
                    model: 85,
                    stepping: 7,
                    microcode: "0x1".to_string(),
                    cpu_mhz: 2500.0,
                    cache_size: 37486592,
                    physical_id: 0,
                    siblings: 2,
                    core_id: 0,
                    cpu_cores: 1,
                    apicid: 0,
                    initial_apicid: 0,
                    fpu: true,
                    fpu_exception: true,
                    cpuid_level: 22,
                    wp: true,
                    flags: [
                        "fpu",
                        "vme",
                        "de",
                        "pse",
                        "tsc",
                        "msr",
                        "pae",
                        "mce",
                        "cx8",
                        "apic",
                        "sep",
                        "mtrr",
                        "pge",
                        "mca",
                        "cmov",
                        "pat",
                        "pse36",
                        "clflush",
                        "mmx",
                        "fxsr",
                        "sse",
                        "sse2",
                        "ss",
                        "ht",
                        "syscall",
                        "nx",
                        "pdpe1gb",
                        "rdtscp",
                        "lm",
                        "constant_tsc",
                        "rep_good",
                        "nopl",
                        "xtopology",
                        "nonstop_tsc",
                        "cpuid",
                        "tsc_known_freq",
                        "pni",
                        "pclmulqdq",
                        "ssse3",
                        "fma",
                        "cx16",
                        "pcid",
                        "sse4_1",
                        "sse4_2",
                        "x2apic",
                        "movbe",
                        "popcnt",
                        "tsc_deadline_timer",
                        "aes",
                        "xsave",
                        "avx",
                        "f16c",
                        "rdrand",
                        "hypervisor",
                        "lahf_lm",
                        "abm",
                        "3dnowprefetch",
                        "invpcid_single",
                        "pti",
                        "fsgsbase",
                        "tsc_adjust",
                        "bmi1",
                        "hle",
                        "avx2",
                        "smep",
                        "bmi2",
                        "erms",
                        "invpcid",
                        "rtm",
                        "mpx",
                        "avx512f",
                        "avx512dq",
                        "rdseed",
                        "adx",
                        "smap",
                        "clflushopt",
                        "clwb",
                        "avx512cd",
                        "avx512bw",
                        "avx512vl",
                        "xsaveopt",
                        "xsavec",
                        "xgetbv1",
                        "xsaves",
                        "arat",
                        "avx512_vnni",
                    ]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                    bugs: [
                        "cpu_meltdown",
                        "spectre_v1",
                        "spectre_v2",
                        "spec_store_bypass",
                        "l1tf",
                        "mds",
                        "swapgs",
                        "taa",
                        "itlb_multihit",
                    ]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                    bogomips: 5000.0,
                    tlb_size: TlbSize { count: 0, unit: 0 },
                    clflush_size: 64,
                    cache_alignment: 64,
                    address_sizes: AddressSizes { phy: 46, virt: 48 },
                    power_management: Vec::<String>::new(),
                };
                assert_eq!(cpu0, cpu_vec[0]);
                assert_eq!(cpus, cpu_vec.len());
            }
            CpuInfo::Arm64(_) => {
                panic!("Should not reach here");
            }
            _ => {
                panic!("Unknown CPU architecture")
            }
        }
    }

    #[test]
    fn test_parse_cpu_info_amd() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("./test_resources/arch/x86_64/amd/cpuinfo");
        let binding = d.into_os_string();
        let cpuinfo_path = binding.to_str().unwrap();

        let cpus = 16;
        let cpu_info = parse_cpuinfo!(cpuinfo_path, "x86_64").unwrap();
        match cpu_info {
            CpuInfo::X86_64(cpu_vec) => {
                let cpu0 = X86_64CpuInfo {
                    processor: 0,
                    vendor_id: "AuthenticAMD".to_string(),
                    model_name: "AMD Ryzen 7 5800 8-Core Processor".to_string(),
                    cpu_family: 25,
                    model: 33,
                    stepping: 0,
                    microcode: "0xa201016".to_string(),
                    cpu_mhz: 2200.0,
                    cache_size: 524288,
                    physical_id: 0,
                    siblings: 16,
                    core_id: 0,
                    cpu_cores: 8,
                    apicid: 0,
                    initial_apicid: 0,
                    fpu: true,
                    fpu_exception: true,
                    cpuid_level: 16,
                    wp: true,
                    flags: [
                        "fpu",
                        "vme",
                        "de",
                        "pse",
                        "tsc",
                        "msr",
                        "pae",
                        "mce",
                        "cx8",
                        "apic",
                        "sep",
                        "mtrr",
                        "pge",
                        "mca",
                        "cmov",
                        "pat",
                        "pse36",
                        "clflush",
                        "mmx",
                        "fxsr",
                        "sse",
                        "sse2",
                        "ht",
                        "syscall",
                        "nx",
                        "mmxext",
                        "fxsr_opt",
                        "pdpe1gb",
                        "rdtscp",
                        "lm",
                        "constant_tsc",
                        "rep_good",
                        "nopl",
                        "nonstop_tsc",
                        "cpuid",
                        "extd_apicid",
                        "aperfmperf",
                        "rapl",
                        "pni",
                        "pclmulqdq",
                        "monitor",
                        "ssse3",
                        "fma",
                        "cx16",
                        "sse4_1",
                        "sse4_2",
                        "movbe",
                        "popcnt",
                        "aes",
                        "xsave",
                        "avx",
                        "f16c",
                        "rdrand",
                        "lahf_lm",
                        "cmp_legacy",
                        "svm",
                        "extapic",
                        "cr8_legacy",
                        "abm",
                        "sse4a",
                        "misalignsse",
                        "3dnowprefetch",
                        "osvw",
                        "ibs",
                        "skinit",
                        "wdt",
                        "tce",
                        "topoext",
                        "perfctr_core",
                        "perfctr_nb",
                        "bpext",
                        "perfctr_llc",
                        "mwaitx",
                        "cpb",
                        "cat_l3",
                        "cdp_l3",
                        "hw_pstate",
                        "ssbd",
                        "mba",
                        "ibrs",
                        "ibpb",
                        "stibp",
                        "vmmcall",
                        "fsgsbase",
                        "bmi1",
                        "avx2",
                        "smep",
                        "bmi2",
                        "erms",
                        "invpcid",
                        "cqm",
                        "rdt_a",
                        "rdseed",
                        "adx",
                        "smap",
                        "clflushopt",
                        "clwb",
                        "sha_ni",
                        "xsaveopt",
                        "xsavec",
                        "xgetbv1",
                        "xsaves",
                        "cqm_llc",
                        "cqm_occup_llc",
                        "cqm_mbm_total",
                        "cqm_mbm_local",
                        "clzero",
                        "irperf",
                        "xsaveerptr",
                        "rdpru",
                        "wbnoinvd",
                        "arat",
                        "npt",
                        "lbrv",
                        "svm_lock",
                        "nrip_save",
                        "tsc_scale",
                        "vmcb_clean",
                        "flushbyasid",
                        "decodeassists",
                        "pausefilter",
                        "pfthreshold",
                        "avic",
                        "v_vmsave_vmload",
                        "vgif",
                        "v_spec_ctrl",
                        "umip",
                        "pku",
                        "ospke",
                        "vaes",
                        "vpclmulqdq",
                        "rdpid",
                        "overflow_recov",
                        "succor",
                        "smca",
                        "fsrm",
                    ]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                    bugs: [
                        "sysret_ss_attrs",
                        "spectre_v1",
                        "spectre_v2",
                        "spec_store_bypass",
                        "srso",
                    ]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                    bogomips: 6787.44,
                    tlb_size: TlbSize {
                        count: 2560,
                        unit: 4096,
                    },
                    clflush_size: 64,
                    cache_alignment: 64,
                    address_sizes: AddressSizes { phy: 48, virt: 48 },
                    power_management: [
                        "ts",
                        "ttp",
                        "tm",
                        "hwpstate",
                        "cpb",
                        "eff_freq_ro",
                        "[13]",
                        "[14]",
                    ]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                };
                assert_eq!(cpu0, cpu_vec[0]);
                assert_eq!(cpus, cpu_vec.len());
            }
            CpuInfo::Arm64(_) => {
                panic!("Should not reach here");
            }
            _ => {
                panic!("Unknown CPU architecture")
            }
        }
    }
}
