use std::sync::Mutex;

use super::*;

use crate::pages::PageKind;

#[test]
pub fn open_one_and_close() {
    let pam = PageAccessManager::empty();

    let guard = pam.open_page_as(42, PageKind::BTreeChild).unwrap();
    assert_eq!(guard.page_index, 42);

    assert_eq!(
        pam.open_pages.lock().unwrap().as_slice(),
        &[OpenPage {
            page_index: 42,
            info: OpenPageInfo {
                used_as: PageKind::BTreeChild,
                uses_minus_one: 0
            }
        }]
    );

    drop(guard);

    assert_eq!(pam.open_pages.lock().unwrap().as_slice(), &[]);
}

#[test]
pub fn open_one_fail_exhausted() {
    let pam = PageAccessManager {
        open_pages: Mutex::new(vec![OpenPage {
            page_index: 42,
            info: OpenPageInfo {
                used_as: PageKind::BTreeChild,
                uses_minus_one: u32::MAX,
            },
        }]),
    };

    assert_eq!(
        pam.open_page_as(42, PageKind::BTreeChild).unwrap_err(),
        PamError::PageUseCountExhausted { page_index: 42 }
    );
}

#[test]
pub fn open_one_fail_invalid_page_access() {
    let pam = PageAccessManager::empty();

    let first = pam.open_page_as(42, PageKind::BTreeChild).unwrap();

    assert_eq!(
        pam.open_page_as(42, PageKind::BTreeRoot).unwrap_err(),
        PamError::InvalidPageAccess {
            page_index: 42,
            page_in_use_as: PageKind::BTreeChild,
            requested: PageKind::BTreeRoot
        }
    );

    drop(first);
}

#[test]
pub fn open_multiple_of_same_kind_and_close() {
    let pam = PageAccessManager::empty();

    let ref_1 = pam
        .open_page_as(4534095832344, PageKind::BTreeRoot)
        .unwrap();

    let ref_2 = pam
        .open_page_as(4534095832344, PageKind::BTreeRoot)
        .unwrap();

    let ref_3 = pam
        .open_page_as(4534095832344, PageKind::BTreeRoot)
        .unwrap();

    assert_eq!(
        pam.open_pages.lock().unwrap().as_slice(),
        &[OpenPage {
            page_index: 4534095832344,
            info: OpenPageInfo {
                used_as: PageKind::BTreeRoot,
                uses_minus_one: 2
            }
        }]
    );

    drop(ref_3);

    assert_eq!(
        pam.open_pages.lock().unwrap().as_slice(),
        &[OpenPage {
            page_index: 4534095832344,
            info: OpenPageInfo {
                used_as: PageKind::BTreeRoot,
                uses_minus_one: 1
            }
        }]
    );

    drop(ref_2);

    assert_eq!(
        pam.open_pages.lock().unwrap().as_slice(),
        &[OpenPage {
            page_index: 4534095832344,
            info: OpenPageInfo {
                used_as: PageKind::BTreeRoot,
                uses_minus_one: 0
            }
        }]
    );

    drop(ref_1);
}

#[test]
pub fn open_multiple_and_close() {
    let pam = PageAccessManager::empty();

    let a = pam.open_page_as(410, PageKind::BTreeChild).unwrap();
    let b = pam.open_page_as(411, PageKind::BTreeChild).unwrap();
    let c = pam.open_page_as(412, PageKind::BTreeRoot).unwrap();

    assert_eq!(
        pam.page_info(410),
        Some(OpenPageInfo {
            used_as: PageKind::BTreeChild,
            uses_minus_one: 0
        })
    );

    assert_eq!(
        pam.page_info(411),
        Some(OpenPageInfo {
            used_as: PageKind::BTreeChild,
            uses_minus_one: 0
        })
    );

    assert_eq!(
        pam.page_info(412),
        Some(OpenPageInfo {
            used_as: PageKind::BTreeRoot,
            uses_minus_one: 0
        })
    );

    drop([a, b, c]);
}
