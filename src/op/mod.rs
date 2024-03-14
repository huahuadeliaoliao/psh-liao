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
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

pub mod common;

use crate::runtime::psh::profiling::{cpu, memory, system};
use crate::runtime::ServerWasiView;

use self::common::cpu_info::parse_cpuinfo;
use self::common::mem_info::parse_meminfo;
use self::common::system::{get_kernel_version, parse_os_version};
use self::common::{AddressSizes, Arm64CpuInfo, TlbSize, X86_64CpuInfo};

impl memory::Host for ServerWasiView {
    fn get_memory_info(&mut self) -> wasmtime::Result<Result<memory::MemoryInfo, String>> {
        let mem_info = parse_meminfo!().unwrap();
        Ok(Ok(memory::MemoryInfo {
            mem_total: mem_info.mem_total,
            mem_free: mem_info.mem_free,
            mem_available: mem_info.mem_available,
            buffers: mem_info.buffers,
            cached: mem_info.cached,
            swap_cached: mem_info.swap_cached,
            active: mem_info.active,
            inactive: mem_info.inactive,
            active_anon: mem_info.active_anon,
            inactive_anon: mem_info.inactive_anon,
            active_file: mem_info.active_file,
            inactive_file: mem_info.inactive_file,
            unevictable: mem_info.unevictable,
            mlocked: mem_info.mlocked,
            swap_total: mem_info.swap_total,
            swap_free: mem_info.swap_free,
            dirty: mem_info.dirty,
            writeback: mem_info.writeback,
            anon_pages: mem_info.anon_pages,
            mapped: mem_info.mapped,
            shmem: mem_info.shmem,
            kreclaimable: mem_info.kreclaimable,
            slab: mem_info.slab,
            sreclaimable: mem_info.sreclaimable,
            sunreclaim: mem_info.sunreclaim,
            kernel_stack: mem_info.kernel_stack,
            page_tables: mem_info.page_tables,
            nfs_unstable: mem_info.nfs_unstable,
            bounce: mem_info.bounce,
            writeback_tmp: mem_info.writeback_tmp,
            commit_limit: mem_info.commit_limit,
            committed_as: mem_info.committed_as,
            vmalloc_total: mem_info.vmalloc_total,
            vmalloc_used: mem_info.vmalloc_used,
            vmalloc_chunk: mem_info.vmalloc_chunk,
            percpu: mem_info.percpu,
            cma_total: mem_info.cma_total,
            cma_free: mem_info.cma_free,
            hardware_corrupted: mem_info.hardware_corrupted,
            anon_huge_pages: mem_info.anon_huge_pages,
            shmem_huge_pages: mem_info.shmem_huge_pages,
            shmem_pmd_mapped: mem_info.shmem_pmd_mapped,
            file_huge_pages: mem_info.file_huge_pages,
            file_pmd_mapped: mem_info.file_pmd_mapped,
            huge_pages_total: mem_info.huge_pages_total,
            huge_pages_free: mem_info.huge_pages_free,
            huge_pages_rsvd: mem_info.huge_pages_rsvd,
            huge_pages_surp: mem_info.huge_pages_surp,
            huge_page_size: mem_info.huge_page_size,
            huge_tlb: mem_info.huge_tlb,
            direct_map4k: mem_info.direct_map4k,
            direct_map2_m: mem_info.direct_map2_m,
            direct_map1_g: mem_info.direct_map1_g,
        }))
    }
}

impl system::Host for ServerWasiView {
    fn os_version(&mut self) -> wasmtime::Result<Option<String>> {
        parse_os_version!().map_err(wasmtime::Error::from)
    }

    fn kernel_version(&mut self) -> wasmtime::Result<String> {
        get_kernel_version().map_err(wasmtime::Error::from)
    }
}

impl<T: AsRef<AddressSizes>> From<T> for cpu::AddressSizes {
    fn from(value: T) -> Self {
        cpu::AddressSizes {
            phy: value.as_ref().phy,
            virt: value.as_ref().virt,
        }
    }
}

impl<T: AsRef<TlbSize>> From<T> for cpu::TlbSize {
    fn from(value: T) -> Self {
        cpu::TlbSize {
            count: value.as_ref().count,
            unit: value.as_ref().unit,
        }
    }
}

impl<T: AsRef<Arm64CpuInfo>> From<T> for cpu::Arm64CpuInfo {
    fn from(value: T) -> Self {
        cpu::Arm64CpuInfo {
            processor: value.as_ref().processor as u32,
            bogomips: value.as_ref().bogomips,
            features: value.as_ref().features.clone(),
            cpu_implementer: value.as_ref().cpu_implementer,
            cpu_architecture: value.as_ref().cpu_architecture,
            cpu_variant: value.as_ref().cpu_variant,
            cpu_part: value.as_ref().cpu_part,
            cpu_revision: value.as_ref().cpu_revision,
            address_sizes: value.as_ref().address_sizes.as_ref().into(),
        }
    }
}

impl<T: AsRef<X86_64CpuInfo>> From<T> for cpu::X64CpuInfo {
    fn from(value: T) -> Self {
        cpu::X64CpuInfo {
            processor: value.as_ref().processor as u32,
            vendor_id: value.as_ref().vendor_id.clone(),
            model_name: value.as_ref().model_name.clone(),
            cpu_family: value.as_ref().cpu_family as u32,
            model: value.as_ref().model as u32,
            stepping: value.as_ref().stepping as u32,
            microcode: value.as_ref().microcode.clone(),
            cpu_mhz: value.as_ref().cpu_mhz,
            cache_size: value.as_ref().cache_size,
            physical_id: value.as_ref().physical_id as u32,
            siblings: value.as_ref().siblings as u32,
            core_id: value.as_ref().core_id as u32,
            cpu_cores: value.as_ref().cpu_cores as u32,
            apicid: value.as_ref().apicid as u32,
            initial_apicid: value.as_ref().initial_apicid as u32,
            fpu: value.as_ref().fpu,
            fpu_exception: value.as_ref().fpu_exception,
            cpuid_level: value.as_ref().cpuid_level as u32,
            wp: value.as_ref().wp,
            flag: value.as_ref().flags.clone(),
            bugs: value.as_ref().bugs.clone(),
            bogomips: value.as_ref().bogomips,
            tlb_size: value.as_ref().tlb_size.as_ref().into(),
            clflush_size: value.as_ref().clflush_size,
            cache_alignment: value.as_ref().cache_alignment,
            address_sizes: value.as_ref().address_sizes.as_ref().into(),
            power_management: value.as_ref().power_management.clone(),
        }
    }
}

impl cpu::Host for ServerWasiView {
    fn get_cpu_info(&mut self) -> wasmtime::Result<Result<cpu::CpuInfo, String>> {
        let cpu_info = parse_cpuinfo!().unwrap();
        let res = match cpu_info {
            common::CPUInfo::X86_64(x64) => {
                Ok(cpu::CpuInfo::X64(x64.iter().map(|x| x.into()).collect()))
            }
            common::CPUInfo::Arm64(arm64) => Ok(cpu::CpuInfo::Arm64(
                arm64.iter().map(|x| x.into()).collect(),
            )),
            common::CPUInfo::Unsupported(unsupported) => Err(unsupported),
        };

        Ok(res)
    }
}
