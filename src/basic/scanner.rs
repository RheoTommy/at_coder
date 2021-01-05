pub struct Scanner {
    buf: std::collections::VecDeque<String>,
    reader: std::io::BufReader<std::io::Stdin>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            buf: std::collections::VecDeque::new(),
            reader: std::io::BufReader::new(std::io::stdin()),
        }
    }

    fn scan_line(&mut self) {
        use std::io::BufRead;
        let mut flag = 0;
        while self.buf.is_empty() {
            let mut s = String::new();
            self.reader.read_line(&mut s).unwrap();
            let mut iter = s.split_whitespace().peekable();
            if iter.peek().is_none() {
                if flag >= 5 {
                    panic!("There is no input!");
                }
                flag += 1;
                continue;
            }

            for si in iter {
                self.buf.push_back(si.to_string());
            }
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        self.scan_line();
        self.buf
            .pop_front()
            .unwrap()
            .parse()
            .unwrap_or_else(|_| panic!("Couldn't parse!"))
    }

    pub fn next_usize(&mut self) -> usize {
        self.next()
    }

    pub fn next_isize(&mut self) -> isize {
        self.next()
    }

    pub fn next_chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }

    pub fn next_string(&mut self) -> String {
        self.next()
    }

    pub fn next_char(&mut self) -> char {
        self.next()
    }

    pub fn next_float(&mut self) -> f64 {
        self.next()
    }
}