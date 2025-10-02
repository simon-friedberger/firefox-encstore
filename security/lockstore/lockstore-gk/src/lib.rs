/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use nserror::{nsresult, NS_ERROR_FAILURE, NS_ERROR_INVALID_ARG, NS_OK};
use nsstring::nsACString;
use thin_vec::ThinVec;


#[no_mangle]
pub extern "C" fn lockstore_test(lockstore_name: &nsACString) -> nsresult {
    // Log the function call
    log::debug!("Entering lockstore_test");

    // Log the name of the lockstore
    log::debug!(" (input) lockstore_name: {:?}", lockstore_name);
    log::info!("Lockstore test successful");
    NS_OK
}
