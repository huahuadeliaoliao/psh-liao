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

use std::io;

use perf_event_rs::{
    config,
    config::{Cpu, Process},
    counting::{Config, Counter, CounterStat},
};

pub fn counter_new(process: &Process, cpu: &Cpu, cfg: &mut Config) -> config::Result<Counter> {
    Counter::new(process, cpu, cfg)
}

pub fn counter_enable(counter: &Counter) -> io::Result<()> {
    counter.enable()
}

pub fn counter_disable(counter: &Counter) -> io::Result<()> {
    counter.disable()
}

pub fn counter_reset(counter: &Counter) -> io::Result<()> {
    counter.reset()
}

pub fn counter_stat(counter: &mut Counter) -> io::Result<CounterStat> {
    counter.stat()
}
