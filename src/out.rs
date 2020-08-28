use super::builder::*;
use super::prompt::*;
use super::util::colors::*;
use super::util::symbols::*;
use super::util::*;

pub fn out(
    width: u32,
    profiles: &Vec<Vec<(&Box<dyn PromptSegment>, LENGTH_LEVEL)>>,
    prompt: &Prompt,
) {
    // output
    for profile in profiles {
        let sum = (&profile)
            .iter()
            .filter(|(seg, level)| (*seg).is_enabled())
            .map(|(seg, level)| (**seg).get_size()[*level as usize] + 1)
            .sum();

        if width >= sum {
            let mut string = String::new();
            string.reserve(1024);

            for (i, &(seg, level)) in profile
                .iter()
                .filter(|(seg, level)| (*seg).is_enabled())
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
