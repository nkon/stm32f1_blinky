
// use core::sync::atomic::AtomicIsize; なぜか使えないので、enum Lock を実装する。

enum Lock {
    Locked,
    Unlocked,
}

const QUEUE_LENGTH: usize = 32;

struct Queue {
    q: [u32; QUEUE_LENGTH],
    length: usize,
    lock: Lock,
}

static mut QUEUE: Queue = Queue {
    q: [0; QUEUE_LENGTH],
    length: 0,
    lock: Lock::Unlocked,
};

impl Queue {
    fn push(&mut self, obj: u32) -> bool {
        if self.length == QUEUE_LENGTH - 1 {
            false
        } else {
            loop {
                match self.lock {
                    Lock::Locked => continue,
                    _ => {
                        self.lock = Lock::Locked;
                        break;
                    }
                }
            }
            // これが無ければビルドエラー(`abort`リンクエラー)
            if self.length < QUEUE_LENGTH {
                self.q[self.length] = obj;
            }
            self.length += 1;
            self.lock = Lock::Unlocked;
            true
        }
    }

    fn pop_match_first(&mut self, mask: u32) -> Option<u32> {
        if self.length == 0 {
            None
        } else if self.length > QUEUE_LENGTH {
            None
        } else {
            loop {
                match self.lock {
                    Lock::Locked => continue,
                    _ => {
                        self.lock = Lock::Locked;
                        break;
                    }
                }
            }
            for i in 0..self.length {
                // これが無ければビルドエラー(`abort`リンクエラー)
                // i < self.length <= QUEUE_LENGTH は見てない?
                if i < QUEUE_LENGTH {
                    // マッチした時は、イベントを消す。
                    if (self.q[i] & 0xffff0000) == (mask & 0xffff0000) {
                        let ret = self.q[i] & 0x0000ffff;
                        if i < self.length {
                            for j in (i + 1)..self.length {
                                if (0 < j) && (j < QUEUE_LENGTH) {
                                    self.q[j - 1] = self.q[j];
                                }
                            }
                        }
                        self.length -= 1;
                        self.lock = Lock::Unlocked;
                        return Some(ret);
                    // そうで無ければ、フラグを落とす。
                    } else if (self.q[i] & mask) != 0 {
                        let ret = self.q[i] & 0x0000ffff;
                        self.q[i] = self.q[i] & !mask;
                        self.lock = Lock::Unlocked;
                        return Some(ret);
                    }
                }
            }
            self.lock = Lock::Unlocked;
            None // 見つからなかった
        }
    }
}


/// イベントを受信する。
/// キューに溜まっているイベントをスキャンして、
/// マスク部を OR して not 0 ならイベント有り。Some(イベント)を返す。
/// マッチするイベントがなければ None を返す。
pub fn catch(mask: u32) -> Option<u32> {
    unsafe { QUEUE.pop_match_first(mask) }
}


/// 宛先を指定せずにイベントを送る。
/// 上位16bitはマスク、下位16bitはイベント値。
pub fn send(mask: u32, event: u32) -> () {
    let obj = (mask & 0xffff0000) | (event & 0x0000ffff);
    unsafe {
        QUEUE.push(obj);
    }
}
