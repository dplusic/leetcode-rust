use std::env;

mod n0394_decode_string;
mod n0457_circular_array_loop_2;
mod n0524_longest_word_in_dictionary_through_deleting;
mod n0679_24_game;
mod n0731_my_calendar_ii;
mod n0837_new_21_game;
mod n0849_maximize_distance_to_closest_person_2;
mod n0850_rectangle_area_ii_2;
mod n0900_rle_iterator;
mod n0911_online_election;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} problem_no", args[0]);
        return;
    }

    let problem_no = args[1].as_str();
    match problem_no {
        "394" => n0394_decode_string::run(),
        "457" => n0457_circular_array_loop_2::run(),
        "524" => n0524_longest_word_in_dictionary_through_deleting::run(),
        "679" => n0679_24_game::run(),
        "731" => n0731_my_calendar_ii::run(),
        "837" => n0837_new_21_game::run(),
        "849" => n0849_maximize_distance_to_closest_person_2::run(),
        "850" => n0850_rectangle_area_ii_2::run(),
        "900" => n0900_rle_iterator::run(),
        "911" => n0911_online_election::run(),
        _ => eprintln!("Not found: {}", problem_no),
    }
}
