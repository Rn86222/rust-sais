struct Data {
    index: usize,
    is_lms: bool,
}

struct Bucket {
    forward_index: i32,
    backward_index: i32,
    size: usize,
    data: Vec<Data>,
}

impl Bucket {
    fn new(size: usize) -> Self {
        let mut data = vec![];
        for _ in 0..size {
            data.push(Data {
                index: 0,
                is_lms: false,
            })
        }
        Bucket {
            forward_index: 0,
            backward_index: size as i32 - 1,
            size,
            data,
        }
    }

    fn forward_insert(&mut self, index: usize, is_lms: bool) {
        assert!(self.forward_index <= self.backward_index);
        self.data[self.forward_index as usize].is_lms = is_lms;
        self.data[self.forward_index as usize].index = index;
        self.forward_index += 1;
    }

    fn backward_insert(&mut self, index: usize, is_lms: bool) {
        assert!(self.forward_index <= self.backward_index);
        self.data[self.backward_index as usize].is_lms = is_lms;
        self.data[self.backward_index as usize].index = index;
        self.backward_index -= 1;
    }
}

struct BucketTable {
    buckets: Vec<Bucket>,
    bucket_exists: Vec<bool>,
    bucket_num: usize,
    string: Vec<usize>,
}

impl BucketTable {
    fn new(counts: Vec<usize>, bucket_num: usize, string: Vec<usize>) -> Self {
        let mut buckets = vec![];
        let bucket_exists = vec![false; bucket_num];
        for i in 0..bucket_num {
            let bucket = Bucket::new(counts[i]);
            buckets.push(bucket);
        }
        BucketTable {
            buckets,
            bucket_exists,
            bucket_num,
            string,
        }
    }

    fn forward_insert(&mut self, index: usize, is_lms: bool) {
        let name = self.string[index];
        self.buckets[name].forward_insert(index, is_lms);
        self.bucket_exists[name] = true;
    }

    fn backward_insert(&mut self, index: usize, is_lms: bool) {
        let name = self.string[index];
        self.buckets[name].backward_insert(index, is_lms);
        self.bucket_exists[name] = true;
    }

    #[allow(dead_code)]
    fn display(&self, length: usize) {
        for i in 0..self.bucket_num {
            if self.bucket_exists[i] {
                for j in 0..self.buckets[i].forward_index {
                    let j = j as usize;
                    for k in self.buckets[i].data[j].index..length {
                        print!("{} ", self.string[k]);
                    }
                    println!(" {}", self.buckets[i].data[j].is_lms as u8);
                }
                for j in (self.buckets[i].backward_index + 1)..(self.buckets[i].size as i32) {
                    let j = j as usize;
                    for k in self.buckets[i].data[j].index..length {
                        print!("{} ", self.string[k]);
                    }
                    println!(" {}", self.buckets[i].data[j].is_lms as u8);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn display_as_suffix_array(&self, length: usize) {
        for i in 1..self.bucket_num {
            if self.bucket_exists[i] {
                for j in 0..self.buckets[i].forward_index {
                    let j = j as usize;
                    for k in self.buckets[i].data[j].index..length {
                        print!("{}", self.string[k] as u8 as char);
                    }
                    println!("");
                }
                for j in (self.buckets[i].backward_index + 1)..(self.buckets[i].size as i32) {
                    let j = j as usize;
                    for k in self.buckets[i].data[j].index..length {
                        print!("{}", self.string[k] as u8 as char);
                    }
                    println!("");
                }
            }
        }
    }

    #[allow(dead_code)]
    fn suffix_array_to_string_vec(&self, length: usize) -> Vec<String> {
        let mut string_vec: Vec<String> = vec![];
        for i in 1..self.bucket_num {
            if self.bucket_exists[i] {
                for j in 0..self.buckets[i].forward_index {
                    let j = j as usize;
                    string_vec.push(
                        self.string[(self.buckets[i].data[j].index)..(length - 1)]
                            .iter()
                            .map(|x| *x as u8 as char)
                            .collect(),
                    );
                }
                for j in (self.buckets[i].backward_index + 1)..(self.buckets[i].size as i32) {
                    let j = j as usize;
                    string_vec.push(
                        self.string[(self.buckets[i].data[j].index)..(length - 1)]
                            .iter()
                            .map(|x| *x as u8 as char)
                            .collect(),
                    );
                }
            }
        }
        string_vec
    }

    fn suffix_array_to_index_vec(&self) -> Vec<usize> {
        let mut index_vec = vec![];
        for i in 1..self.bucket_num {
            if self.bucket_exists[i] {
                for j in 0..self.buckets[i].forward_index {
                    let j = j as usize;
                    index_vec.push(self.buckets[i].data[j].index);
                }
                for j in (self.buckets[i].backward_index + 1)..(self.buckets[i].size as i32) {
                    let j = j as usize;
                    index_vec.push(self.buckets[i].data[j].index);
                }
            }
        }
        index_vec
    }
}

const S: bool = false;
const L: bool = true;

#[derive(Clone, Copy)]
struct SuffixInfo {
    suffix_type: bool,
    is_lms: bool,
}

fn induced_sort(bt: &mut BucketTable, table: Vec<SuffixInfo>, length: usize) {
    // L-type の位置を決定
    for i in 0..bt.bucket_num {
        if bt.bucket_exists[i] {
            let mut j: i32 = 0;
            while (j as i32) < bt.buckets[i].forward_index {
                let suffix_index = bt.buckets[i].data[j as usize].index;
                if suffix_index > 0 && table[suffix_index - 1].suffix_type == L {
                    bt.forward_insert(suffix_index - 1, table[suffix_index - 1].is_lms);
                }
                j += 1;
            }
            j = bt.buckets[i].backward_index + 1;
            while j < bt.buckets[i].size as i32 {
                let suffix_index = bt.buckets[i].data[j as usize].index;
                if suffix_index > 0 && table[suffix_index - 1].suffix_type == L {
                    bt.forward_insert(suffix_index - 1, table[suffix_index - 1].is_lms);
                }
                j += 1;
            }
        }
    }

    // S-type(LMS)を削除（実際にはb_idxをsize - 1とすればよい）
    // ただし最後のLMSは削除してはいけない
    for i in 0..bt.bucket_num {
        if bt.bucket_exists[i] && bt.buckets[i].data[0].index != length - 1 {
            bt.buckets[i].backward_index = bt.buckets[i].size as i32 - 1;
            if bt.buckets[i].forward_index == 0 {
                bt.bucket_exists[i] = false;
            }
        }
    }

    // S-type の位置を決定
    for i in (0..bt.bucket_num).rev() {
        if bt.bucket_exists[i] {
            let mut j: i32 = bt.buckets[i].size as i32 - 1;
            while (j as i32) >= bt.buckets[i].backward_index + 1 {
                let suffix_index = bt.buckets[i].data[j as usize].index;
                if suffix_index > 0 && table[suffix_index - 1].suffix_type == S {
                    bt.backward_insert(suffix_index - 1, table[suffix_index - 1].is_lms);
                }
                j -= 1;
            }
            j = bt.buckets[i].forward_index - 1;
            while j >= 0 {
                let suffix_index = bt.buckets[i].data[j as usize].index;
                if suffix_index > 0 && table[suffix_index - 1].suffix_type == S {
                    bt.backward_insert(suffix_index - 1, table[suffix_index - 1].is_lms);
                }
                j -= 1;
            }
        }
    }
}

fn construst_suffix_array(
    bt: &mut BucketTable,
    string: &Vec<usize>,
    length: usize,
    origin: bool,
    counts: Vec<usize>,
) {
    if length <= 1 {
        bt.forward_insert(0, false);
        return;
    }
    let mut table = vec![
        SuffixInfo {
            suffix_type: S,
            is_lms: false,
        };
        length
    ]; // 各 suffix を管理するための表
    let mut lms_ids = vec![];
    let mut lms_index_to_id = vec![0; length];
    let mut lms_num = 0;
    table[0].is_lms = false;
    table[length - 1].suffix_type = S;

    // table の作成
    for i in 0..length {
        if i > 0 {
            if string[length - i - 1] < string[length - i] {
                table[length - i - 1].suffix_type = S;
            } else if string[length - i - 1] > string[length - i] {
                table[length - i - 1].suffix_type = L;
            } else {
                table[length - i - 1].suffix_type = table[length - i].suffix_type;
            }
        }
        table[length - i - 1].is_lms = i < length - 1
            && table[length - i - 2].suffix_type == L
            && table[length - i - 1].suffix_type == S;
    }
    for i in 1..length {
        table[i].is_lms = table[i - 1].suffix_type == L && table[i].suffix_type == S;
        if table[i].is_lms {
            lms_ids.push(i);
            lms_index_to_id[i] = lms_num;
            lms_num += 1;
        }
    }

    let mut bt_lms_substring; // LMS-substring の番号づけ用の Bucket Table
    if origin {
        bt_lms_substring = BucketTable::new(counts, 128, (*string).clone());
    } else {
        bt_lms_substring = BucketTable::new(counts, length, (*string).clone())
    }

    /*
      (1) LMS を先頭の1文字についてバケットソート
      (2) induced_sort() 実行
      (3) induced_sort() 後のLMSの順番に、対応する LMS-substring に番号をつける(0,1,2,...)
      (4) この数字を文字だと思って、元の文字列に出現していた順に並べて文字列を作る(例. 2310)
      (5) この文字列に対し construct_suffix_array()
      (6) 作成した suffix array から順に対応する LMS を取り出し(LMS のソート)バケットテーブルに入れる
      (7) induced_sort() 実行
      終わり
    */

    // (1) LMS を先頭の1文字についてバケットソート
    for i in 0..lms_num {
        bt_lms_substring.backward_insert(lms_ids[i], true);
    }

    // (2) induced_sort() 実行
    induced_sort(&mut bt_lms_substring, table.clone(), length);

    // (3) induced_sort() 後のLMSの順番に、対応する- LMS-substring に番号をつける(0,1,2,...)
    let mut lms_index_to_substring_index = vec![0; length];

    let mut cnt = 0;
    let mut before_substring: &[usize] = &[];
    let mut next_counts = vec![0; lms_num];
    for i in 0..bt_lms_substring.bucket_num {
        if bt_lms_substring.bucket_exists[i] {
            for j in (bt_lms_substring.buckets[i].backward_index + 1)
                ..(bt_lms_substring.buckets[i].size as i32)
            {
                let j = j as usize;
                if bt_lms_substring.buckets[i].data[j].is_lms {
                    if lms_index_to_id[bt_lms_substring.buckets[i].data[j].index] < lms_num - 1 {
                        let current_string = &string[(bt_lms_substring.buckets[i].data[j].index)
                            ..(lms_ids
                                [lms_index_to_id[bt_lms_substring.buckets[i].data[j].index] + 1])];
                        if cnt == 0 || *before_substring != *current_string {
                            lms_index_to_substring_index
                                [bt_lms_substring.buckets[i].data[j].index] = cnt;
                            next_counts[cnt] += 1;
                            cnt += 1;
                        } else {
                            lms_index_to_substring_index
                                [bt_lms_substring.buckets[i].data[j].index] = cnt - 1;
                            next_counts[cnt - 1] += 1;
                        }
                        before_substring = current_string;
                    } else {
                        lms_index_to_substring_index[bt_lms_substring.buckets[i].data[j].index] =
                            cnt;
                        next_counts[cnt] += 1;
                        cnt += 1;
                    }
                }
            }
        }
    }

    // (4) この数字を文字だと思って、元の文字列に出現していた順に並べて文字列を作る(例. 2310)
    // 実際には usize の配列である(例. {2,3,1,0})
    let mut sorted_substring_ids = vec![0; lms_num];
    let mut substring_first_to_lms_index = vec![0; lms_num]; // substring の開始位置から対応する LMS の index を求める
    for i in 0..lms_num {
        sorted_substring_ids[i] = lms_index_to_substring_index[lms_ids[i]];
        substring_first_to_lms_index[i] = lms_ids[i];
    }

    // (5) この文字列に対し construct_suffix_array() を再帰呼び出し
    let mut bt_lms_sort =
        BucketTable::new(next_counts.clone(), lms_num, sorted_substring_ids.clone());
    construst_suffix_array(
        &mut bt_lms_sort,
        &sorted_substring_ids,
        lms_num,
        false,
        next_counts,
    );

    // (6) 作成した suffix array から順に対応するLMSを取り出し(LMSのソート)バケットテーブルに入れる
    // このときは取り出した順とは逆に各バケットの下から入れていく

    // まず LMS を順に取り出して sorted_lms_ids に入れる
    let mut sorted_lms_ids = vec![0; lms_num];
    cnt = 0;
    for i in 0..bt_lms_sort.bucket_num {
        if bt_lms_sort.bucket_exists[i] {
            for j in 0..bt_lms_sort.buckets[i].forward_index {
                let j = j as usize;
                sorted_lms_ids[cnt] =
                    substring_first_to_lms_index[bt_lms_sort.buckets[i].data[j].index];
                cnt += 1;
            }
            for j in
                (bt_lms_sort.buckets[i].backward_index + 1)..(bt_lms_sort.buckets[i].size as i32)
            {
                let j = j as usize;
                sorted_lms_ids[cnt] =
                    substring_first_to_lms_index[bt_lms_sort.buckets[i].data[j].index];
                cnt += 1;
            }
        }
    }

    // 取り出した順とは逆に各バケットの下から入れる
    for i in (0..lms_num).rev() {
        bt.backward_insert(sorted_lms_ids[i], true);
    }

    // (7) induced_sort() 実行
    induced_sort(bt, table, length);
}

pub fn sais(string: &String) -> Vec<usize> {
    let length = string.len() + 1; // 末尾の'\0'を考慮して1足しておく
    let mut counts = vec![0; 128];
    let mut usize_string: Vec<usize> = string
        .chars()
        .map(|c| {
            counts[c as usize] += 1;
            c as usize
        })
        .collect();
    usize_string.push('\0' as usize);
    counts[0] = 1; // '\0'の出現回数は1

    // suffix array(完成)用の Bucket Table
    let mut bt_suffix_array = BucketTable::new(counts.clone(), 128, usize_string.clone());

    construst_suffix_array(&mut bt_suffix_array, &usize_string, length, true, counts);
    bt_suffix_array.suffix_array_to_index_vec()
}
