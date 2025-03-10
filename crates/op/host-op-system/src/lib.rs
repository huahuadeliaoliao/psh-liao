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

mod cpu;
mod disk;
mod interrupt;
mod memory;
mod network;
mod os;
mod process;
mod rps;
mod vmstat;

use std::sync::Arc;

use psh_system::{
    System,
    cpu::CpuHandle,
    disk::DiskHandle,
    interrupt::InterruptHandle,
    memory::MemoryHandle,
    network::NetworkHandle,
    os::OsHandle,
    process::{Process, ProcessHandle},
    rps::RpsHandle,
    vmstat::VmstatHandle,
};
use wasmtime::component::{Linker, ResourceTable};

pub type HostProc = Arc<Process>;

wasmtime::component::bindgen!({
    path: "../../../psh-sdk-wit/wit/deps/system",
    world: "imports",
    with: {
        "profiling:system/process/process": HostProc,
    },
    // https://github.com/bytecodealliance/wasmtime/pull/8310
    // wasmtime have added a config in bindgen! macro to allow user specify
    // whether they want a function be able to trap(outer wasmtime::Result).
    // by default the value is false, we use true here to compatible with our
    // previous implementations.
    trappable_imports: [
        "[method]process.pid",
        "[method]process.cmd",
        "[method]process.exe",
        "[method]process.environ",
        "[method]process.cwd",
        "[method]process.root",
        "[method]process.user-id",
        "all",
        "current",
    ],
});

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SysCtx {
    table: ResourceTable,
    system: System,
    os: OsHandle,
    cpu: CpuHandle,
    disk: DiskHandle,
    memory: MemoryHandle,
    process: ProcessHandle,
    rps: RpsHandle,
    network: NetworkHandle,
    interrupt: InterruptHandle,
    vmstat: VmstatHandle,
}

pub fn add_to_linker<T>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut SysCtx) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    crate::Imports::add_to_linker(l, f)
}
