
use lock::Lock;

const QUEUE_LENGTH: usize = 32;

#[derive(Debug)]
struct Queue {
    q: [u32; QUEUE_LENGTH],
    len: usize,
    lock: Lock,
}

static mut EVENT_QUEUE: Queue = Queue {
    q: [0; QUEUE_LENGTH],
    len: 0,
    lock: Lock::Unlocked,
};

impl Queue {
    fn push(&mut self, obj: u32) -> bool {
        if self.len >= QUEUE_LENGTH - 1 {
            false
        } else {
            self.lock.get_lock();
            self.q[self.len] = obj;
            self.len += 1;
            self.lock.unlock();
            true
        }
    }

    fn pop_match_first(&mut self, mask: u32) -> Option<u32> {
        if self.len == 0 {
            None
        } else if self.len > QUEUE_LENGTH {
            None
        } else {
            self.lock.get_lock();
            for i in 0..self.len {
                if (self.q[i] & mask) != 0 {          // マスクがマッチしたら
                    self.q[i] = self.q[i] & !mask;    // ビットを落とす
                    let ret = self.q[i] & 0x0000ffff; // イベントを返す
                    if self.q[i] & 0xffff0000 == 0 {  // マスクが空なら
                        if i < self.len {          // キューを詰める
                            for j in (i + 1)..self.len {
                                if (0 < j) && (j < QUEUE_LENGTH) {
                                    self.q[j - 1] = self.q[j];
                                }
                            }
                        }
                        self.len -= 1;
                    }
                    self.lock.unlock();
                    return Some(ret);
                }
            }
            self.lock.unlock();
            None // 見つからなかった
        }
    }
}

/// イベントを受信する。
/// キューに溜まっているイベントをスキャンして、
/// マスク部を OR して not 0 ならイベント有り。Some(イベント)を返す。
/// マッチするイベントがなければ None を返す。
pub fn catch(mask: u32) -> Option<u32> {
    unsafe {
        EVENT_QUEUE.pop_match_first(mask)
    }
}


/// 宛先を指定せずにイベントを送る。
/// 上位16bitはマスク、下位16bitはイベント値。
pub fn send(mask: u32, event: u32) -> () {
    let obj = (mask & 0xffff0000) | (event & 0x0000ffff);
    unsafe {
        EVENT_QUEUE.push(obj);
    }
}
