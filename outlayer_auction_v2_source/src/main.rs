#![no_main]

#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern "C" {
    fn fd_read(
        fd: i32,
        iovs: *const Iovec,
        iovs_len: i32,
        nread: *mut i32,
    ) -> i32;

    fn fd_write(
        fd: i32,
        iovs: *const Ciovec,
        iovs_len: i32,
        nwritten: *mut i32,
    ) -> i32;
}

#[repr(C)]
struct Iovec {
    buf: *mut u8,
    buf_len: usize,
}

#[repr(C)]
struct Ciovec {
    buf: *const u8,
    buf_len: usize,
}

fn write_stdout(bytes: &[u8]) {
    let iov = Ciovec {
        buf: bytes.as_ptr(),
        buf_len: bytes.len(),
    };

    let mut written = 0;

    unsafe {
        fd_write(1, &iov, 1, &mut written);
    }
}

#[no_mangle]
pub extern "C" fn _start() {
    let mut input = [0u8; 1024];

    let iov = Iovec {
        buf: input.as_mut_ptr(),
        buf_len: input.len(),
    };

    let mut nread = 0;

    unsafe {
        fd_read(0, &iov, 1, &mut nread);
    }

    let input = &input[..nread as usize];

    let mut bids = [0u32; 4];
    let mut count = 0usize;
    let mut current = 0u32;
    let mut reading_number = false;

    for &c in input {
        if c.is_ascii_digit() {
            current = current * 10 + (c - b'0') as u32;
            reading_number = true;
        } else if reading_number {
            if count < 4 {
                bids[count] = current;
                count += 1;
            }
            current = 0;
            reading_number = false;
        }
    }

    if reading_number && count < 4 {
        bids[count] = current;
        count += 1;
    }

    if count != 4 {
        write_stdout(b"{\"error\":\"expected exactly four bids\"}\n");
        return;
    }

    let mut highest = bids[0];
    let mut second = 0u32;
    let mut winner = 0usize;

    for i in 1..4 {
        let bid = bids[i];

        if bid > highest {
            second = highest;
            highest = bid;
            winner = i;
        } else if bid > second {
            second = bid;
        }
    }

    let mut output = [0u8; 128];

    let prefix = b"{\"winner\":";
    let middle = b",\"price\":";
    let suffix = b"}\n";

    let mut pos = 0;

    for &b in prefix {
        output[pos] = b;
        pos += 1;
    }

    pos += write_number(&mut output[pos..], winner as u32);

    for &b in middle {
        output[pos] = b;
        pos += 1;
    }

    pos += write_number(&mut output[pos..], second);

    for &b in suffix {
        output[pos] = b;
        pos += 1;
    }

    write_stdout(&output[..pos]);
}

fn write_number(buf: &mut [u8], mut n: u32) -> usize {
    if n == 0 {
        buf[0] = b'0';
        return 1;
    }

    let mut tmp = [0u8; 10];
    let mut len = 0usize;

    while n > 0 {
        tmp[len] = b'0' + (n % 10) as u8;
        n /= 10;
        len += 1;
    }

    for i in 0..len {
        buf[i] = tmp[len - 1 - i];
    }

    len
}
