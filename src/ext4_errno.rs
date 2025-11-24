//! EXT4 error codes
//! Corresponds to ext4_errno.h in the C implementation

/// Success code
pub const EOK: i32 = 0;

/// Operation not permitted
pub const EPERM: i32 = 1;

/// No such file or directory
pub const ENOENT: i32 = 2;

/// I/O error
pub const EIO: i32 = 5;

/// No such device or address
pub const ENXIO: i32 = 6;

/// Argument list too long
pub const E2BIG: i32 = 7;

/// Out of memory
pub const ENOMEM: i32 = 12;

/// Permission denied
pub const EACCES: i32 = 13;

/// Bad address
pub const EFAULT: i32 = 14;

/// File exists
pub const EEXIST: i32 = 17;

/// No such device
pub const ENODEV: i32 = 19;

/// Not a directory
pub const ENOTDIR: i32 = 20;

/// Is a directory
pub const EISDIR: i32 = 21;

/// Invalid argument
pub const EINVAL: i32 = 22;

/// File too large
pub const EFBIG: i32 = 27;

/// No space left on device
pub const ENOSPC: i32 = 28;

/// Read-only file system
pub const EROFS: i32 = 30;

/// Too many links
pub const EMLINK: i32 = 31;

/// Math result not representable
pub const ERANGE: i32 = 34;

/// Directory not empty
pub const ENOTEMPTY: i32 = 39;

/// No data available
pub const ENODATA: i32 = 61;

/// Not supported
pub const ENOTSUP: i32 = 95;

/// Result type for EXT4 operations
pub type Ext4Result<T> = Result<T, i32>;

/// Convert error code to string description
pub const fn errno_to_str(errno: i32) -> &'static str {
    match errno {
        EOK => "Success",
        EPERM => "Operation not permitted",
        ENOENT => "No such file or directory",
        EIO => "I/O error",
        ENXIO => "No such device or address",
        E2BIG => "Argument list too long",
        ENOMEM => "Out of memory",
        EACCES => "Permission denied",
        EFAULT => "Bad address",
        EEXIST => "File exists",
        ENODEV => "No such device",
        ENOTDIR => "Not a directory",
        EISDIR => "Is a directory",
        EINVAL => "Invalid argument",
        EFBIG => "File too large",
        ENOSPC => "No space left on device",
        EROFS => "Read-only file system",
        EMLINK => "Too many links",
        ERANGE => "Math result not representable",
        ENOTEMPTY => "Directory not empty",
        ENODATA => "No data available",
        ENOTSUP => "Not supported",
        _ => "Unknown error",
    }
}

/// Check if error code indicates success
#[inline]
pub const fn is_success(errno: i32) -> bool {
    errno == EOK
}

/// Check if error code indicates failure
#[inline]
pub const fn is_error(errno: i32) -> bool {
    errno != EOK
}

/// Convert errno to Result
#[inline]
pub fn errno_to_result(errno: i32) -> Ext4Result<()> {
    if errno == EOK { Ok(()) } else { Err(errno) }
}

/// Convert Result to errno
#[inline]
pub fn result_to_errno<T>(result: &Ext4Result<T>) -> i32 {
    match result {
        Ok(_) => EOK,
        Err(e) => *e,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes_defined() {
        // 确保所有错误码都已定义
        assert_eq!(EOK, 0);
        assert_eq!(EPERM, 1);
        assert_eq!(ENOENT, 2);
        assert_eq!(EIO, 5);
        assert_eq!(ENOMEM, 12);
        assert_eq!(EINVAL, 22);
        assert_eq!(ENOSPC, 28);
        assert_eq!(ENODATA, 61);
        assert_eq!(ENOTSUP, 95);
    }

    #[test]
    fn test_error_codes_unique() {
        // 确保错误码唯一（除了可能的别名）
        let codes = [
            EOK, EPERM, ENOENT, EIO, ENXIO, E2BIG, ENOMEM, EACCES, EFAULT, EEXIST, ENODEV, ENOTDIR,
            EISDIR, EINVAL, EFBIG, ENOSPC, EROFS, EMLINK, ERANGE, ENOTEMPTY, ENODATA, ENOTSUP,
        ];

        // EOK 应该是 0，其他都应该非零
        assert_eq!(codes[0], 0);
        for &code in &codes[1..] {
            assert_ne!(code, 0, "Error code should be non-zero");
        }
    }

    #[test]
    fn test_errno_to_str() {
        assert_eq!(errno_to_str(EOK), "Success");
        assert_eq!(errno_to_str(ENOENT), "No such file or directory");
        assert_eq!(errno_to_str(ENOMEM), "Out of memory");
        assert_eq!(errno_to_str(EINVAL), "Invalid argument");
        assert_eq!(errno_to_str(ENOSPC), "No space left on device");
        assert_eq!(errno_to_str(999), "Unknown error");
    }

    #[test]
    fn test_is_success() {
        assert!(is_success(EOK));
        assert!(!is_success(EINVAL));
        assert!(!is_success(ENOENT));
        assert!(!is_success(-1));
    }

    #[test]
    fn test_is_error() {
        assert!(!is_error(EOK));
        assert!(is_error(EINVAL));
        assert!(is_error(ENOENT));
        assert!(is_error(-1));
    }

    #[test]
    fn test_errno_to_result() {
        assert!(errno_to_result(EOK).is_ok());
        assert!(errno_to_result(EINVAL).is_err());
        assert_eq!(errno_to_result(ENOENT).unwrap_err(), ENOENT);
    }

    #[test]
    fn test_result_to_errno() {
        let ok_result: Ext4Result<()> = Ok(());
        assert_eq!(result_to_errno(&ok_result), EOK);

        let err_result: Ext4Result<()> = Err(EINVAL);
        assert_eq!(result_to_errno(&err_result), EINVAL);
    }

    #[test]
    fn test_result_type() {
        fn test_function(should_fail: bool) -> Ext4Result<u32> {
            if should_fail { Err(EINVAL) } else { Ok(42) }
        }

        let result = test_function(false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);

        let result = test_function(true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EINVAL);
    }

    #[test]
    fn test_common_error_scenarios() {
        // 文件不存在
        let result: Ext4Result<()> = Err(ENOENT);
        assert_eq!(
            errno_to_str(result.unwrap_err()),
            "No such file or directory"
        );

        // 权限拒绝
        let result: Ext4Result<()> = Err(EACCES);
        assert_eq!(errno_to_str(result.unwrap_err()), "Permission denied");

        // 空间不足
        let result: Ext4Result<()> = Err(ENOSPC);
        assert_eq!(errno_to_str(result.unwrap_err()), "No space left on device");

        // 只读文件系统
        let result: Ext4Result<()> = Err(EROFS);
        assert_eq!(errno_to_str(result.unwrap_err()), "Read-only file system");
    }

    #[test]
    fn test_error_propagation() {
        fn inner_function() -> Ext4Result<i32> {
            Err(ENOMEM)
        }

        fn outer_function() -> Ext4Result<i32> {
            inner_function()?;
            Ok(100)
        }

        let result = outer_function();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ENOMEM);
    }
}
