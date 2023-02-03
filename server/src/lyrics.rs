#[derive(Debug)]
pub struct Line {
    pub t: i32,
    pub s: String,
}

#[derive(Debug)]
pub struct Lyrics {
    pub lines: Vec<Line>, 
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseLineError {
    EmptyLine,
    BadTimestamp,
    NoTimestamp,
}

#[derive(Debug)]
pub struct ParseLyricError {
    pub e: ParseLineError,
    pub line_no: usize,
}


impl Line {
    /// Parse a line of lyrics
    pub fn parse(s: &str) -> Result<Self, ParseLineError> {

        let s = s.trim(); 

        if s.len() == 0 {
            return Err(ParseLineError::EmptyLine);
        }

        let v: Vec<_> = s.splitn(2, ']').collect();
        if v.len() != 2 {
            return Err(ParseLineError::NoTimestamp);
        }

        let t = v[0];
        if &t[..'['.len_utf8()] != "[" {
            return Err(ParseLineError::BadTimestamp);
        }
        let t = &t['['.len_utf8()..];
        let l = v[1];

        let v: Vec<_> = t.splitn(2, ':').collect();
        if v.len() != 2 {
            return Err(ParseLineError::BadTimestamp);
        }
        let m: i32 = v[0].parse().map_err(|_| ParseLineError::BadTimestamp)?;
        let s: f32 = v[1].parse().map_err(|_| ParseLineError::BadTimestamp)?;
        let ms: i32 = (s * 1000.0) as i32;

        return Ok(Line {
            t: m * 60 * 1000 + ms,
            s: l.to_string()
        })
    }
}

impl Lyrics {
    /// Parse a whole lyrics
    pub fn parse(s: &str) -> Result<Self, ParseLyricError> {

        let mut lines: Vec<Line> = Vec::new();
        
        for (i, sline) in s.lines().enumerate() {
            match Line::parse(sline) {
                Ok(line) => {
                    if let Some(last_line) = lines.last_mut() {
                        if last_line.t == line.t {
                            last_line.s += &line.s;
                            continue;
                        }
                    }
                    lines.push(line);
                },
                Err(e) => {
                    match e {
                        ParseLineError::EmptyLine => continue,
                        e => return Err(ParseLyricError { e, line_no: i + 1 }),
                    }
                }
            }
        }

        if lines.len() != 0 && lines.last().unwrap().s == "纯音乐，请欣赏" {
            return Ok(Lyrics { lines: Vec::new() });
        }

        Ok(Lyrics { lines })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let s = "\n[00:12.34]aaa\n\n[01:14.12]\n\n";
        let lrc = Lyrics::parse(s).unwrap();
        
        assert_eq!(lrc.lines.len(), 2);
        assert_eq!(lrc.lines[0].t, 12340);
        assert_eq!(lrc.lines[0].s, "aaa");
        assert_eq!(lrc.lines[1].t, 74120);
        assert_eq!(lrc.lines[1].s, "");
    }

    #[test]
    fn test2() {
        let s = "00:12.32]bbb";
        let e = Lyrics::parse(s).err().unwrap();
        assert_eq!(e.e, ParseLineError::BadTimestamp);
        assert_eq!(e.line_no, 1);
    }

    #[test]
    fn test3() {
        let s = "[0012.32]bbb";
        let e = Lyrics::parse(s).err().unwrap();
        assert_eq!(e.e, ParseLineError::BadTimestamp);
        assert_eq!(e.line_no, 1);
    }

    #[test]
    fn test4() {
        let s = "[0012.32bbb";
        let e = Lyrics::parse(s).err().unwrap();
        assert_eq!(e.e, ParseLineError::NoTimestamp);
        assert_eq!(e.line_no, 1);
    }
}
