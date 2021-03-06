// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use flexi_logger::Logger;
use std::sync::Once;

static INIT_LOG: Once = Once::new();

pub fn initialize() {
    INIT_LOG.call_once(|| {
        Logger::try_with_env().unwrap().start().unwrap();
    });
}
