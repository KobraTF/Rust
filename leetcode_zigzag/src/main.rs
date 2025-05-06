impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows:usize = num_rows as usize;
        let mut answ: Vec<String> = Vec::new();
        if num_rows == 1 {
            return s
        };
        for _ in 0..num_rows {
            let mut st = String::new();
            answ.push(st);
        }
        let mut counter:usize = 0;
        let mut flag = true;
       for ch in s.chars() {
        if counter == 0 && flag{
            flag = false;
            answ[counter].push(ch);
            counter += 1;
        } else if counter == num_rows-1 && flag != true {
            flag = true;
            answ[counter].push(ch);
            counter -= 1;
        } else if flag {
            answ[counter].push(ch);
            counter -= 1;
        } else if flag != true {
            answ[counter].push(ch);
            counter += 1;
        }
       }
       let mut output = String::new();
       for row in answ {
            for ch in row.chars() {
                output.push(ch);
            }
       }
       output
    }
}