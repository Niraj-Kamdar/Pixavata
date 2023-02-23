pub mod wrap;
// pub mod avatar;
pub mod color;
pub mod components;

pub use wrap::*;

use color::*;
use itoa::Buffer;


pub fn render_male(args: ArgsRenderMale) -> String {
    let skin_color = to_rgb(&components::SKIN_COLORS[args.skin_color as usize]);
    let hair_color = to_rgb(&components::HAIR_COLORS[args.hair_color as usize])
        .brighter_or_darker_than(&skin_color, 17.0);
    let eyes_color = to_rgb(&components::EYE_COLORS[args.eyes_color as usize]);
    let eyebrows_color = hair_color
        .darker_than(&skin_color, 7.0)
        .darker_than(&hair_color, 10.0);
    let mustache_color = hair_color
        .darker_than(&skin_color, 7.0)
        .with_alpha(1.0);
    let mouth_color = to_rgb(&components::male::MOUTH_COLORS[args.mouth_color as usize])
        .brighter_or_darker_than(&skin_color, 10.0);
    let glasses_color = to_rgb(&components::GLASSES_COLORS[args.glasses_color as usize]);
    let clothes_color = to_rgb(&components::male::CLOTHES_COLORS[args.clothes_color as usize]);
    let hat_color = to_rgb(&components::male::HAT_COLORS[args.hat_color as usize]);

    let mouth = match args.mood {
        Mood::Sad => components::male::mouths::SAD,
        Mood::Happy => components::male::mouths::HAPPY,
        _ => components::male::mouths::HAPPY,
    };

    let _mustache = components::male::MUSTACHE[(args.mustache as usize) % components::male::MUSTACHE.len()];
    let _glasses = components::male::GLASSES[(args.glasses as usize) % components::male::GLASSES.len()];
    let _hair = components::male::HAIR[(args.hair as usize) % components::male::HAIR.len()];
    let _hat = components::HAT[(args.hat as usize) % components::HAT.len()];
    let _eyes = components::EYES[(args.eyes as usize) % components::EYES.len()];
    let _eyebrows = components::EYEBROWS[(args.eyebrows as usize) % components::EYEBROWS.len()];
    let _clothes = components::male::CLOTHES[(args.clothes as usize) % components::male::CLOTHES.len()];

    let mut svg = [
        components::SVG_START,
        components::male::HEAD,
        _eyes,
        _eyebrows,
        _mustache,
        mouth,
        _glasses,
        _clothes,
        _hair,
        _hat,
        components::SVG_END,
    ]
    .join("");

    svg = svg.replace("${skinColor}", &skin_color.html());
    svg = svg.replace("${hairColor}", &hair_color.html());
    svg = svg.replace("${eyesColor}", &eyes_color.html());
    svg = svg.replace("${eyebrowsColor}", &eyebrows_color.html());
    svg = svg.replace("${mustacheColor}", &mustache_color.html());
    svg = svg.replace(
        "${mustacheColorAlpha}",
        Buffer::new().format(mustache_color.a),
    );
    svg = svg.replace("${mouthColor}", &mouth_color.html());
    svg = svg.replace("${glassesColor}", &glasses_color.html());
    svg = svg.replace("${clothesColor}", &clothes_color.html());
    svg = svg.replace("${hatColor}", &hat_color.html());

    svg.trim().to_string()
}

pub fn render_female(args: ArgsRenderFemale) -> String {
    let skin_color = to_rgb(&components::SKIN_COLORS[args.skin_color as usize]);
    let hair_color = to_rgb(&components::HAIR_COLORS[args.hair_color as usize])
        .brighter_or_darker_than(&skin_color, 17.0);
    let eyes_color = to_rgb(&components::EYE_COLORS[args.eyes_color as usize]);
    let eyebrows_color = hair_color
        .darker_than(&skin_color, 7.0)
        .darker_than(&hair_color, 10.0);
    let accessories_colors = to_rgb(&components::female::ACCESSORIES_COLORS[args.accessories_color as usize]);
    let mouth_color = to_rgb(&components::male::MOUTH_COLORS[args.mouth_color as usize])
        .brighter_or_darker_than(&skin_color, 10.0);
    let glasses_color = to_rgb(&components::GLASSES_COLORS[args.glasses_color as usize]);
    let clothes_color = to_rgb(&components::male::CLOTHES_COLORS[args.clothes_color as usize]);
    let hat_color = to_rgb(&components::male::HAT_COLORS[args.hat_color as usize]);

    let mouth = match args.mood {
        Mood::Sad => components::male::mouths::SAD,
        Mood::Happy => components::male::mouths::HAPPY,
        _ => components::male::mouths::HAPPY,
    };

    let _accessories = components::female::ACCESSORIES[(args.accessories as usize) % components::female::ACCESSORIES.len()];
    let _glasses = components::female::GLASSES[(args.glasses as usize) % components::female::GLASSES.len()];
    let _hair = components::female::HAIR[(args.hair as usize) % components::female::HAIR.len()];
    let _hat = components::HAT[(args.hat as usize) % components::HAT.len()];
    let _eyes = components::EYES[(args.eyes as usize) % components::EYES.len()];
    let _eyebrows = components::EYEBROWS[(args.eyebrows as usize) % components::EYEBROWS.len()];
    let _clothes = components::male::CLOTHES[(args.clothes as usize) % components::male::CLOTHES.len()];

    let mut svg = [
        components::SVG_START,
        components::female::HEAD,
        _eyes,
        _eyebrows,
        _accessories,
        mouth,
        _glasses,
        _clothes,
        _hair,
        _hat,
        components::SVG_END,
    ]
    .join("");

    svg = svg.replace("${skinColor}", &skin_color.html());
    svg = svg.replace("${hairColor}", &hair_color.html());
    svg = svg.replace("${eyesColor}", &eyes_color.html());
    svg = svg.replace("${eyebrowsColor}", &eyebrows_color.html());
    svg = svg.replace("${accessoriesColor}", &accessories_colors.html());
    svg = svg.replace("${mouthColor}", &mouth_color.html());
    svg = svg.replace("${glassesColor}", &glasses_color.html());
    svg = svg.replace("${clothesColor}", &clothes_color.html());
    svg = svg.replace("${hatColor}", &hat_color.html());

    svg.trim().to_string()
}
