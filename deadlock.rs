type Deadlock = Option<uint>;

static mut lock: Deadlock = None;
static mut anotherLock: Deadlock = None;

fn grab_lock(id: uint) {
    unsafe {
	while (lock.is_some()) {
	    ;
	}
	lock = Some(id);
    }
}

fn grab_anotherLock(id: uint) {
    unsafe {
	while (anotherLock.is_some()) {
	    ;
	}
	anotherLock = Some(id);
    }
}

fn release_lock() {
    unsafe {
	lock = None;
    }
}

fn release_anotherLock() {
    unsafe {
	anotherLock = None;
    }
}

fn update_process_1(id: uint) {
    unsafe {
	//while (update(id + 1) != id + 1) {
	  //  ;
	//}
	grab_anotherLock(id);
	println("Process 1 grabbed lock 2");
	release_lock();
    }
}

fn update_process_2(id: uint) {
    grab_lock(id);
    println("Process 2 grabbed lock 1");
    release_anotherLock();
}

fn main() {
    grab_lock(1);
    grab_anotherLock(2);
    do spawn {
	update_process_1(1);
	update_process_2(2);
    }
}
