
use lock::Lock;
use hal;

const QUEUE_LENGTH: usize = 32;

#[derive(Clone,Copy)]
struct Event {
    tick: u32,
    ev: u32,
}

struct Queue {
    q: [Event; QUEUE_LENGTH],
    length: usize,
    lock: Lock,
}

static mut QUEUE: Queue = Queue {
    q: [Event{tick:0,ev:0}; QUEUE_LENGTH],
    length: 0,
    lock: Lock::Unlocked,
};

impl Queue {
    fn push(&mut self, obj: Event) -> bool {
        if self.length >= QUEUE_LENGTH - 1 {
            false
        } else {
            self.lock.get_lock();
            // これが無ければビルドエラー(`abort`リンクエラー)
            if self.length < QUEUE_LENGTH {
                self.q[self.length] = obj;
            }
            self.length += 1;
            self.lock.unlock();
            true
        }
    }

    fn pop_after(&mut self, time: u32) -> Option<u32> {
        if self.length == 0 {
            None
        } else if self.length > QUEUE_LENGTH {
            None
        } else {
            self.lock.get_lock();
            for i in 0..self.length {
                // これが無ければビルドエラー(`abort`リンクエラー)
                // i < self.length <= QUEUE_LENGTH は見てない? static mut だから?
                if i < QUEUE_LENGTH {
                    if self.q[i].tick <= time {
                        let ret = self.q[i].ev;
                        if i < self.length {
                            for j in (i + 1)..self.length {
                                if (0 < j) && (j < QUEUE_LENGTH) {
                                    self.q[j - 1] = self.q[j];
                                }
                            }
                        }
                        self.length -= 1;
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


/// イベントを受信する。
/// キューに溜まっているイベントをスキャンして、
/// マスク部を OR して not 0 ならイベント有り。Some(イベント)を返す。
/// マッチするイベントがなければ None を返す。
pub fn check_event(time: u32) -> Option<u32> {
    unsafe { QUEUE.pop_after(time) }
}


/// 宛先を指定せずにイベントを送る。
/// 上位16bitはマスク、下位16bitはイベント値。
pub fn send(delay: u32, mask: u32, event :u32) -> () {
    let obj = (mask & 0xffff0000) | (event & 0x0000ffff);
    unsafe {
        QUEUE.push(Event{tick: delay + hal::GetTick(), ev:obj});
    }
}

