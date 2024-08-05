mod support;
// use support::*; In case you get bored of the support:: ___ syntax
fn main() {
    // support::shadowing_and_scope();
    // support::mutability_and_dereferencing()
    // support::mutable_and_immutable_string()
    // support::tuple_destructuring()

    // let mut a = 10;
    // let mut b = 20;

    // support::add(&mut a, &mut b);
    // support::copy_numbers()
    // support::functional_ownership()
    // support::ownership_example()

    // support::borrow_mutable_reference()
    // support::possible_error_or_not();
    // support::definitely_an_error();

    // support::address_and_references();
    //  support::auto_dereferencing();
    // support::auto_dereferencing_example();

    // support::array_introduction();
    // support::playing_with_array()
    // support::second_game();
    // support::optimised_playing_with_array();
    // support::dynamic_array()

    // support::shadowing()
    // support::control_flow()
    // support::loops();

    // support::matching_cases()

    let quit_message = support::Message::Quit;
    let color_change = support::Message::ChangeColor(255, 0, 0);
    let move_message = support::Message::Move { x: 10, y: 20 };
    let write_message = support::Message::Write(String::from("Aur vedhiya"));

    support::match_enum(quit_message);
    support::match_enum(color_change);
    support::match_enum(move_message);
    support::match_enum(write_message);
}
