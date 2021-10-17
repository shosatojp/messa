use crate::builder::*;
use crate::config::ProfileConfig;
use crate::config::Segment;
use crate::util::colors::*;
use crate::util::symbols::*;
use crate::util::*;

pub fn out(width: u32, profiles: &Vec<ProfileConfig>, prompt: &Segment) {
    out_line(width, profiles);

    print!(
        "{} ",
        prompt
            .construct(LengthLevel::LONG, BuildMode::CONSTRUCT)
            .data
    );
}

fn out_line(width: u32, profiles: &Vec<ProfileConfig>) {
    for profile in profiles {
        let mut left_sum = 0;
        let mut right_sum = 0;

        for seg in &profile.segments {
            if seg.segment.is_enabled() {
                match seg.location {
                    Location::LEFT => left_sum += seg.segment.get_size()[seg.size as usize] + 1,
                    Location::RIGHT => right_sum += seg.segment.get_size()[seg.size as usize] + 1,
                }
            }
        }

        let sum = left_sum + right_sum;

        if width >= sum {
            // left
            let mut string = String::new();
            string.reserve(1024);

            for (i, seg) in profile
                .segments
                .iter()
                .filter(|seg| seg.segment.is_enabled() && seg.location == Location::LEFT)
                .enumerate()
            {
                string.push_str(background(&seg.segment.get_bg()).as_str());
                if i != 0 {
                    string.push(SYMBOL_RIGHT);
                }
                string.push_str(forground(&seg.segment.get_fg()).as_str());
                string.push_str(
                    (*seg.segment)
                        .construct(seg.size, BuildMode::CONSTRUCT)
                        .data
                        .as_str(),
                );
                string.push_str(forground(&seg.segment.get_bg()).as_str());
            }
            string.push_str(resetbackground().as_str());
            string.push(SYMBOL_RIGHT);
            string.push_str(resetcolor().as_str());

            // right
            let mut right_string = String::new();

            for seg in profile
                .segments
                .iter()
                .filter(|seg| seg.segment.is_enabled() && seg.location == Location::RIGHT)
            {
                right_string.push_str(forground(&seg.segment.get_bg()).as_str());
                right_string.push(SYMBOL_LEFT);
                right_string.push_str(forground(&seg.segment.get_fg()).as_str());
                right_string.push_str(background(&seg.segment.get_bg()).as_str());
                right_string.push_str(
                    seg.segment
                        .construct(seg.size, BuildMode::CONSTRUCT)
                        .data
                        .as_str(),
                );
            }
            right_string.push_str(resetcolor().as_str());

            string.push_str(" ".repeat(width as usize - sum as usize).as_str());
            string.push_str(right_string.as_str());

            println!("{}", string);
            break;
        }
    }
}
