use super::builder::*;
use super::prompt::*;
use super::util::colors::*;
use super::util::symbols::*;
use super::util::*;

pub fn out(
    width: u32,
    profiles: &Vec<Vec<(&Box<dyn PromptSegment>, LENGTH_LEVEL, Location)>>,
    prompt: &Prompt,
) {
    // output
    for profile in profiles {
        let mut left_sum = 0;
        let mut right_sum = 0;

        for (seg, level, loc) in (&profile)
            .iter()
            .filter(|(seg, level, loc)| (*seg).is_enabled())
        {
            match loc {
                Location::LEFT => left_sum += (*seg).get_size()[*level as usize] + 1,
                Location::RIGHT => right_sum += (*seg).get_size()[*level as usize] + 1,
            }
        }

        let sum = left_sum + right_sum;

        if width >= sum {
            // left
            let mut string = String::new();
            string.reserve(1024);

            for (i, &(seg, level, loc)) in profile
                .iter()
                .filter(|(seg, level, loc)| (*seg).is_enabled() && *loc == Location::LEFT)
                .enumerate()
            {
                string.push_str(background((*seg).get_bg()).as_str());
                if i != 0 {
                    string.push(SYMBOL_RIGHT);
                }
                string.push_str(forground((*seg).get_fg()).as_str());
                string.push_str((*seg).construct(level, BuildMode::CONSTRUCT).data.as_str());
                string.push_str(forground((*seg).get_bg()).as_str());
            }
            string.push_str(resetbackground().as_str());
            string.push(SYMBOL_RIGHT);
            string.push_str(resetcolor().as_str());

            // right
            let mut right_string = String::new();

            for (seg, level, loc) in profile
                .iter()
                .filter(|(seg, level, loc)| *loc == Location::RIGHT)
            {
                right_string.push_str(forground(seg.get_bg()).as_str());
                right_string.push(SYMBOL_LEFT);
                right_string.push_str(forground(seg.get_fg()).as_str());
                right_string.push_str(background(seg.get_bg()).as_str());
                right_string.push_str(seg.construct(*level, BuildMode::CONSTRUCT).data.as_str());
            }
            right_string.push_str(resetcolor().as_str());

            string.push_str(" ".repeat(width as usize - sum as usize).as_str());
            string.push_str(right_string.as_str());

            println!("{}", string);
            break;
        }
    }

    print!(
        "{} ",
        prompt
            .construct(LENGTH_LEVEL::LONG, BuildMode::CONSTRUCT)
            .data
    );
}
