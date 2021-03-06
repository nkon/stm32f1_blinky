
use lock::Lock;
use hal;

const QUEUE_LENGTH: usize = 32;

#[derive(Clone, Copy, Debug)]
struct Event {
    tick: u32,
    ev: u32,
}

#[derive(Debug)]
struct Queue {
    q: [Event; QUEUE_LENGTH],
    len: usize,
    lock: Lock,
}

static mut DELAY_QUEUE: Queue = Queue {
    q: [Event{tick:0,ev:0}; QUEUE_LENGTH],
    len: 0,
    lock: Lock::Unlocked,
};

impl Queue {
    /// 末尾に要素を追加する。
    fn push(&mut self, obj: Event) -> bool {
        if self.len >= QUEUE_LENGTH - 1 {
            false
        } else {
            self.lock.get_lock();
            // これが無ければビルドエラー(`abort`リンクエラー)
            if self.len < QUEUE_LENGTH {
                self.q[self.len] = obj;
            }
            self.len += 1;
            self.lock.unlock();
            true
        }
    }

    fn sort_insert(&mut self, obj: Event) -> bool {
        if self.len >= QUEUE_LENGTH {
            false
        } else if self.len == 0 {
            self.push(obj);
            true
        } else {
            self.lock.get_lock();
            let mut i = self.len-1;
            loop {
                if is_after(self.q[i].tick, obj.tick) {
                    self.q[i+1] = obj;
                    break;
                } else {
                    self.q[i+1] = self.q[i];
                    if i == 0 {
                        self.q[0] = obj;
                        break;
                    } else {
                        i -= 1;
                    }
                }
            }
            self.len += 1;
            self.lock.unlock();
            true
        }
    }

    fn pop_after(&mut self, time: u32) -> Option<u32> {
        if self.len == 0 {
            None
        } else if self.len > QUEUE_LENGTH {
            None
        } else {
            self.lock.get_lock();
            for i in 0..self.len {
                // これが無ければビルドエラー(`abort`リンクエラー)
                // i < self.len <= QUEUE_LENGTH は見てない? static mut だから?
                if i < QUEUE_LENGTH {
                    if is_after(self.q[i].tick, time) {
                        let ret = self.q[i].ev;
                        if i < self.len {    // キューを詰める
                            for j in (i + 1)..self.len {
                                if (0 < j) && (j < QUEUE_LENGTH) {
                                    self.q[j - 1] = self.q[j];
                                }
                            }
                        }
                        self.len -= 1;
                        self.lock.unlock();
                        return Some(ret);
                    }
                }
            }
            self.lock.unlock();
            None // 見つからなかった
        }
    }
}

/// now が target より後なら true を返す。 
fn is_after(target:u32, now:u32) -> bool {
    if (target < 0x4000_0000) && (now >= 0xc000_0000) {
        false
    } else if target >= 0xc000_0000 {
        if now < 0x8000_0000 {
            true
        } else if target < now {
            true
        } else {
            false
        }
    } else if target < now {
        true
    } else {
        false
    }
}


/// キューに溜まっているイベントをスキャンして、
/// time(通常は現在時刻 `hal::GetTick()`が渡される)が超過していたらイベント有り。Some(イベント)を返す。
/// マッチするイベントがなければ None を返す。
pub fn check_event(time: u32) -> Option<u32> {
    unsafe { DELAY_QUEUE.pop_after(time) }
}


/// delay[ms]後に向かってイベントを送る
/// delay < 0x4000_0000 のこと!!!
pub fn send(delay: u32, mask: u32, event :u32) -> () {
    let obj = (mask & 0xffff_0000) | (event & 0x0000_ffff);
    if delay >= 0x4000_0000 {
        return;
    }
    unsafe {
        DELAY_QUEUE.sort_insert(Event{tick: delay + hal::GetTick(), ev:obj});
    }
}

