pub fn run() {
    run_one(
        &[(10, 20), (50, 60), (10, 40), (5, 15), (5, 10), (25, 55)],
        &[true, true, true, false, true, true],
    );
    run_one(
        &[
            (24, 40),
            (43, 50),
            (27, 43),
            (5, 21),
            (30, 40),
            (14, 29),
            (3, 19),
            (3, 14),
            (25, 39),
            (6, 19),
        ],
        &[
            true, true, true, true, false, false, true, false, false, false,
        ],
    );
}

fn run_one(books: &[(i32, i32)], expected: &[bool]) {
    let mut obj = MyCalendarTwo::new();
    let res: Vec<bool> = books
        .iter()
        .map(|&(start, end)| obj.book(start, end))
        .collect();
    assert_eq!(res, expected);
}

/////////////////////

use std::collections::BTreeMap;

type Start = i32;

#[derive(Debug, PartialEq, Clone)]
struct TimeSlot {
    start: i32,
    end: i32,
    booked_count: i8,
}

struct MyCalendarTwo {
    time_slots: BTreeMap<Start, TimeSlot>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            time_slots: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let intersected_time_slots = get_intersected_time_slots(&self.time_slots, start, end);
        if is_finished(&intersected_time_slots) {
            insert_time_slot(&mut self.time_slots, start, end, 1);
            true
        } else if has_double_booked(&intersected_time_slots) {
            false
        } else {
            let mut booked_time_slots = book_intersected(start, end, intersected_time_slots);
            self.time_slots.append(&mut booked_time_slots);
            true
        }
    }
}

#[inline(always)]
fn get_intersected_time_slots(
    time_slots: &BTreeMap<Start, TimeSlot>,
    start: i32,
    end: i32,
) -> std::iter::Chain<
    std::option::IntoIter<(&Start, &TimeSlot)>,
    std::collections::btree_map::Range<Start, TimeSlot>,
> {
    time_slots
        .range(..start)
        .next_back()
        .filter(|(_, x)| start < x.end)
        .into_iter()
        .chain(time_slots.range(start..end))
}

#[inline(always)]
fn book_intersected<'a, I>(
    start: i32,
    end: i32,
    intersected_time_slots: I,
) -> BTreeMap<Start, TimeSlot>
where
    I: Iterator<Item = (&'a Start, &'a TimeSlot)>,
{
    let mut booked_time_slots: BTreeMap<Start, TimeSlot> = BTreeMap::new();

    let mut time_slot_start_to_be_booked = start;

    for (_, intersected_time_slot) in intersected_time_slots {
        if intersected_time_slot.start < time_slot_start_to_be_booked {
            insert_time_slot(
                &mut booked_time_slots,
                intersected_time_slot.start,
                time_slot_start_to_be_booked,
                1,
            );

            if intersected_time_slot.end <= end {
                insert_time_slot(
                    &mut booked_time_slots,
                    time_slot_start_to_be_booked,
                    intersected_time_slot.end,
                    2,
                );
            } else {
                // end < intersected_time_slot.end
                insert_time_slot(&mut booked_time_slots, time_slot_start_to_be_booked, end, 2);
                insert_time_slot(&mut booked_time_slots, end, intersected_time_slot.end, 1);
            }
        } else {
            // time_slot_start_to_be_booked <= intersected_time_slot.start

            if time_slot_start_to_be_booked < intersected_time_slot.start {
                insert_time_slot(
                    &mut booked_time_slots,
                    time_slot_start_to_be_booked,
                    intersected_time_slot.start,
                    1,
                );
            }

            if intersected_time_slot.end <= end {
                insert_time_slot(
                    &mut booked_time_slots,
                    intersected_time_slot.start,
                    intersected_time_slot.end,
                    2,
                );
            } else {
                // end < intersected_time_slot.end
                insert_time_slot(&mut booked_time_slots, intersected_time_slot.start, end, 2);
                insert_time_slot(&mut booked_time_slots, end, intersected_time_slot.end, 1);
            }
        }

        time_slot_start_to_be_booked = intersected_time_slot.end;
    }

    if time_slot_start_to_be_booked < end {
        insert_time_slot(&mut booked_time_slots, time_slot_start_to_be_booked, end, 1);
    }

    booked_time_slots
}

#[inline(always)]
fn insert_time_slot(
    time_slots: &mut BTreeMap<Start, TimeSlot>,
    start: i32,
    end: i32,
    booked_count: i8,
) {
    time_slots.insert(
        start,
        TimeSlot {
            start,
            end,
            booked_count,
        },
    );
}

#[inline(always)]
fn has_double_booked<'a, I>(intersected_time_slots: &I) -> bool
where
    I: Iterator<Item = (&'a Start, &'a TimeSlot)> + Clone,
{
    intersected_time_slots.clone().any(
        |(
            _,
            &TimeSlot {
                start: _,
                end: _,
                booked_count,
            },
        )| { booked_count == 2 },
    )
}

#[inline(always)]
fn is_finished<I, T>(iter: &I) -> bool
where
    I: Iterator<Item = T> + Clone,
{
    iter.clone().next().is_none()
}

/////

#[test]
fn test_book_intersected_1() {
    let mut exist: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut exist, 10, 20, 1);
    let actual = book_intersected(10, 30, exist.range(10..30));

    let mut expected: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut expected, 10, 20, 2);
    insert_time_slot(&mut expected, 20, 30, 1);

    assert_eq!(actual, expected);
}

#[test]
fn test_book_intersected_2() {
    let mut exist: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut exist, 10, 20, 1);
    let actual = book_intersected(0, 30, exist.range(0..30));

    let mut expected: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut expected, 0, 10, 1);
    insert_time_slot(&mut expected, 10, 20, 2);
    insert_time_slot(&mut expected, 20, 30, 1);

    assert_eq!(actual, expected);
}

#[test]
fn test_book_intersected_3() {
    let mut exist: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut exist, 10, 30, 1);
    let actual = book_intersected(20, 40, exist.range(..40));

    let mut expected: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut expected, 10, 20, 1);
    insert_time_slot(&mut expected, 20, 30, 2);
    insert_time_slot(&mut expected, 30, 40, 1);

    assert_eq!(actual, expected);
}

#[test]
fn test_book_intersected_4() {
    let mut exist: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut exist, 20, 40, 1);
    let actual = book_intersected(10, 30, exist.range(10..30));

    let mut expected: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut expected, 10, 20, 1);
    insert_time_slot(&mut expected, 20, 30, 2);
    insert_time_slot(&mut expected, 30, 40, 1);

    assert_eq!(actual, expected);
}

#[test]
fn test_book_intersected_5() {
    let mut exist: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut exist, 20, 40, 1);
    let actual = book_intersected(20, 40, exist.range(20..40));

    let mut expected: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut expected, 20, 40, 2);

    assert_eq!(actual, expected);
}

#[test]
fn test_book_intersected_6() {
    let mut exist: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut exist, 20, 50, 1);
    let actual = book_intersected(30, 40, exist.range(..40));

    let mut expected: BTreeMap<Start, TimeSlot> = BTreeMap::new();
    insert_time_slot(&mut expected, 20, 30, 1);
    insert_time_slot(&mut expected, 30, 40, 2);
    insert_time_slot(&mut expected, 40, 50, 1);

    assert_eq!(actual, expected);
}

/*
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
