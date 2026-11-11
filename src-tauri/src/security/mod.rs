//! Protected local secret storage.
//! On Windows uses DPAPI (CryptProtectData / CryptUnprotectData).
//! Non-Windows falls back to an explicit error (desktop target is Windows 11).

use crate::error::{AppError, AppResult};

#[cfg(windows)]
pub fn protect_secret(plaintext: &[u8]) -> AppResult<Vec<u8>> {
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::LocalFree;
    use windows::Win32::Security::Cryptography::{
        CryptProtectData, CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN,
    };

    let mut in_blob = CRYPT_INTEGER_BLOB {
        cbData: plaintext.len() as u32,
        pbData: plaintext.as_ptr() as *mut u8,
    };
    let mut out_blob = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    let ok = unsafe {
        CryptProtectData(
            &mut in_blob,
            PCWSTR::null(),
            None,
            None,
            None,
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut out_blob,
        )
    };

    if ok.is_err() {
        return Err(AppError::Message(
            "DPAPI CryptProtectData failed — cannot store secret".into(),
        ));
    }

    let slice = unsafe {
        std::slice::from_raw_parts(out_blob.pbData, out_blob.cbData as usize)
    };
    let data = slice.to_vec();
    unsafe {
        let _ = LocalFree(windows::Win32::Foundation::HLOCAL(out_blob.pbData as _));
    }
    Ok(data)
}

#[cfg(windows)]
pub fn unprotect_secret(protected: &[u8]) -> AppResult<Vec<u8>> {
    use windows::Win32::Foundation::LocalFree;
    use windows::Win32::Security::Cryptography::{
        CryptUnprotectData, CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN,
    };

    let mut in_blob = CRYPT_INTEGER_BLOB {
        cbData: protected.len() as u32,
        pbData: protected.as_ptr() as *mut u8,
    };
    let mut out_blob = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    let ok = unsafe {
        CryptUnprotectData(
            &mut in_blob,
            None,
            None,
            None,
            None,
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut out_blob,
        )
    };

    if ok.is_err() {
        return Err(AppError::Message(
            "DPAPI CryptUnprotectData failed — secret unreadable on this Windows profile".into(),
        ));
    }

    let slice = unsafe {
        std::slice::from_raw_parts(out_blob.pbData, out_blob.cbData as usize)
    };
    let data = slice.to_vec();
    unsafe {
        let _ = LocalFree(windows::Win32::Foundation::HLOCAL(out_blob.pbData as _));
    }
    Ok(data)
}

#[cfg(not(windows))]
pub fn protect_secret(_plaintext: &[u8]) -> AppResult<Vec<u8>> {
    Err(AppError::NotSupported(
        "Protected secret storage requires Windows DPAPI".into(),
    ))
}

#[cfg(not(windows))]
pub fn unprotect_secret(_protected: &[u8]) -> AppResult<Vec<u8>> {
    Err(AppError::NotSupported(
        "Protected secret storage requires Windows DPAPI".into(),
    ))
}

#[cfg(test)]
mod tests {
    #[cfg(windows)]
    #[test]
    fn dpapi_roundtrip() {
        let secret = b"test-bind-phrase";
        let protected = super::protect_secret(secret).expect("protect");
        assert_ne!(protected, secret);
        let plain = super::unprotect_secret(&protected).expect("unprotect");
        assert_eq!(plain, secret);
    }
}
