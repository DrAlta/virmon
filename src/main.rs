#![allow(unused_imports)]
use std::{collections::HashMap, str::SplitAsciiWhitespace};

use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::complete::{digit1, multispace0, multispace1},
    sequence::{delimited, tuple},
    IResult,
};

trait AttackUsedBy<'c> {
    fn add_used_by(&mut self, key: &'c str, thingie: &'c str);
}
impl<'b> AttackUsedBy<'b> for HashMap<&'b str, Vec<&'b str>> {
    fn add_used_by(&mut self, key: &'b str, value: &'b str) {
        if let Some(x) = self.get_mut(&key) {
            x.push(value);
        } else {
            self.insert(key, vec![value]);
        }
    }
}

#[derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
}

#[derive(Debug)]
enum Faction {
    Hexite,
    Magnamod,
    Velocitron,
}
#[derive(Debug)]
enum BloodType {
    Blue,
    Green,
    Yellow,
}
#[allow(dead_code)]
#[derive(Debug)]
struct Virmon<'a> {
    asset_type_id: u8,
    asset_type_version: u8,
    name: &'a str,
    collector_id: &'a str,
    wave: u8,
    rarity: Rarity,
    faction: Faction,
    blood_type: BloodType,
    level: u8,
    base_speed: u8,
    base_strength: u8,
    base_armor: u8,
    base_health: u8,
    max_speed: u8,
    max_strength: u8,
    max_armor: u8,
    max_health: u8,
    buff_formula_speed: Vec<i32>,
    buff_formula_strength: Vec<i32>,
    buff_formula_armor: Vec<i32>,
    buff_formula_health: Vec<i32>,
    asset_flag: u8,
    swarm_value: i32,
    attacks: Vec<&'a str>,
}
impl<'a> Virmon<'a> {
    fn new(
        asset_type_id: u8,
        asset_type_version: u8,
        name: &'a str,
        collector_id: &'a str,
        wave: u8,
        rarity: Rarity,
        faction: Faction,
        blood_type: BloodType,
        level: u8,
        base_speed: u8,
        base_strength: u8,
        base_armor: u8,
        base_health: u8,
        max_speed: u8,
        max_strength: u8,
        max_armor: u8,
        max_health: u8,
        buff_formula_speed: Vec<i32>,
        buff_formula_strength: Vec<i32>,
        buff_formula_armor: Vec<i32>,
        buff_formula_health: Vec<i32>,
        asset_flag: u8,
        swarm_value: i32,
        attacks: Vec<&'a str>,
    ) -> Self {
        Virmon {
            asset_type_id,
            asset_type_version,
            name,
            collector_id,
            wave,
            rarity,
            faction,
            blood_type,
            level,
            base_speed,
            base_strength,
            base_armor,
            base_health,
            max_speed,
            max_strength,
            max_armor,
            max_health,
            buff_formula_speed,
            buff_formula_strength,
            buff_formula_armor,
            buff_formula_health,
            asset_flag,
            swarm_value,
            attacks,
        }
    }
    fn get_name(&self) -> &str {
    self.name
  }
}
#[allow(dead_code)]
fn print_attacks(attack_descs: HashMap<&str, &str>, attack_used_by: HashMap<&str, Vec<&str>>) {
    for (id, _attack) in &attack_descs {
        println!("{} used by {:?}", id, attack_used_by.get(id).unwrap());
    }
}
fn main() {
  /*
    println!(
        "ass id:{:?}",
        parse_asset_type_id(r#"<asset-type-id>79</asset-type-id>"#)
    );
    println!(
        "ass ver:{:?}",
        parse_asset_type_version(r#"<asset-type-version>0</asset-type-version>"#)
    );

    println!(
        "attack:{:?}",
        parse_attack(
            r#"<attack id="165">

            <attack-name>Fireplow</attack-name>
      
            <type>Melee</type>
      
            <short-in-combat-description>A fire damage attack</short-in-combat-description>
      
            <hh-description>A fire damage attack</hh-description>
      
            <pc-description>A fire damage attack.</pc-description>
      
            <override-description/>
      
            <hack-description/>
      
            <force-damage>Normal</force-damage>
      
            <keyword-1>Fire</keyword-1>
      
            <keyword-2>Close</keyword-2>
      
            <cost>2</cost>
      
            <ammo>0</ammo>
      
            <attack-statement id="11">
      
              <attack-statement-name>40 DAM</attack-statement-name>
      
              <type>Attack</type>
      
              <target>Enemy active Virmon</target>
      
              <stat>Health</stat>
      
              <verb>-</verb>
      
              <delay>0</delay>
      
              <duration>None</duration>
      
              <repetition>None</repetition>
      
              <base-damage-formula>40 * ( $MyStrength / 100 )</base-damage-formula>
      
              <statement-formula>$StatementBaseDamage - $EnemyArmor</statement-formula>
      
            </attack-statement>
      
          </attack>
      "#
        )
    );
    
    println!(
        "buff:{:?}",
        parse_buff_formula(
            r#"<buff-formula-strength>57400</buff-formula-strength>"#,
            "strength"
        )
    );
    println!("what does the stuff after this do?");

  println!(
    "frac:{:?}",
    parse_faction(r#"<faction>Velocitron</faction>"#)
  );

*/
println!("{:#?}", parse_virmon(get_data()));
/*    let mut unprocessed = get_data();

    

    let nanovors = Vec::<Virmon>::new();
    let _attack_descs = HashMap::<&str, &str>::new();
    let mut attack_used_by = HashMap::<&str, Vec<&str>>::new();
    loop {
      let (mut cleaned, _) = multispace0::<_, nom::error::Error<_>>(unprocessed).unwrap();
      //println!("cleaned:{:?}", cleaned);
    
        let Ok((remainder, (nanovor, hys_attacks))) = parse_virmon(cleaned) else {
          break;
        };
        println!("{:#?}", nanovor.get_name());
        for (attack_id, attack_body) in &hys_attacks {
            attack_used_by.add_used_by(attack_id, nanovor.name);
        }
        unprocessed = remainder;
        //println!("{}", unprocessed);
    }
    */
}

fn parse_virmon(input: &str) -> IResult<&str, (Virmon, Vec<(&str, &str)>)> {
    let (mut input, _) = tag("<virmon>")(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, asset_type_id) = parse_asset_type_id(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, asset_type_version) = parse_asset_type_version(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, name) = parse_name(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, collector_id) = parse_collector_id(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, wave) = parse_wave(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, rarity) = parse_rarity(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, faction) = parse_faction(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, blood_type) = parse_blood_type(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, level) = parse_level(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, base_speed) = parse_base_speed(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, base_strength) = parse_base_strength(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, base_armor) = parse_base_armor(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, base_health) = parse_base_health(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, max_speed) = parse_max_speed(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, max_strength) = parse_max_strength(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, max_armor) = parse_max_armor(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, max_health) = parse_max_health(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, buff_formula_speed) = parse_buff_formula(input, "speed")?;
    (input, _) = multispace0(input)?;
    let (mut input, buff_formula_strength) = parse_buff_formula(input, "strength")?;
    (input, _) = multispace0(input)?;
    let (mut input, buff_formula_armor) = parse_buff_formula(input, "armor")?;
    (input, _) = multispace0(input)?;
    let (mut input, buff_formula_health) = parse_buff_formula(input, "health")?;
    (input, _) = multispace0(input)?;
    let (mut input, asset_flag) = parse_asset_flag(input)?;
    (input, _) = multispace0(input)?;


    let (mut input, swarm_value) = parse_swarm_value(input)?;
    (input, _) = multispace0(input)?;


    let (mut input, attacks) = parse_attacks(input)?;
    (input, _) = multispace0(input)?;
    let mut attack_ids = Vec::<&str>::new();
    for (id, _) in &attacks {
      attack_ids.push(id);
    };
    let my_virmon = Virmon::new(
        asset_type_id,
        asset_type_version,
        name,
        collector_id,
        wave,
        rarity,
        faction,
        blood_type,
        level,
        base_speed,
        base_strength,
        base_armor,
        base_health,
        max_speed,
        max_strength,
        max_armor,
        max_health,
        buff_formula_speed,
        buff_formula_strength,
        buff_formula_armor,
        buff_formula_health,
        asset_flag,
        swarm_value,
        attack_ids,
    );
    let (tail, _) = tag("</virmon>")(input)?;
    //println!("tail:{}", tail);
    Ok((tail, (my_virmon, attacks)))
}

fn parse_asset_type_id(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("<asset-type-id>"), digit1, tag("</asset-type-id>")),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_asset_type_version(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(
            tag("<asset-type-version>"),
            digit1,
            tag("</asset-type-version>"),
        ),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    delimited(tag("<name>"), take_till(|c| c == '<'), tag("</name>"))(input)
}

fn parse_collector_id(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("<collector-id>"),
        take_till(|c| c == '<'),
        tag("</collector-id>"),
    )(input)
}

fn parse_wave(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("<wave>"), digit1, tag("</wave>")),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_a_rarity(input: &str) -> IResult<&str, Rarity> {
    if let Ok((tail, _)) = tag::<_, _, nom::error::Error<_>>("common")(input) {
        return Ok((tail, Rarity::Common));
    }
    if let Ok((tail, _)) = tag::<_, _, nom::error::Error<_>>("uncommon")(input) {
        return Ok((tail, Rarity::Uncommon));
    }
    if let Ok((tail, _)) = tag::<_, _, nom::error::Error<_>>("rare")(input) {
        return Ok((tail, Rarity::Rare));
    }
    core::result::Result::Err(nom::Err::Error(nom::error::Error::new(
        "",
        nom::error::ErrorKind::Tag,
    )))
}

fn parse_rarity(input: &str) -> IResult<&str, Rarity> {
    delimited(tag("<rarity>"), parse_a_rarity, tag("</rarity>"))(input)
}

fn parse_a_faction(input: &str) -> IResult<&str, Faction> {
    if let Ok((tail, _)) = tag::<_, _, nom::error::Error<_>>("Hexite")(input) {
        return Ok((tail, Faction::Hexite));
    }
    if let Ok((tail, _)) = tag::<_, _, nom::error::Error<_>>("Magnamod")(input) {
        return Ok((tail, Faction::Magnamod));
    }
    if let Ok((tail, _)) = tag::<_, _, nom::error::Error<_>>("Velocitron")(input) {
        return Ok((tail, Faction::Velocitron));
    }
    core::result::Result::Err(nom::Err::Error(nom::error::Error::new(
        "",
        nom::error::ErrorKind::Tag,
    )))
}

fn parse_faction(input: &str) -> IResult<&str, Faction> {
    delimited(tag("<faction>"), parse_a_faction, tag("</faction>"))(input)
}

fn parse_a_blood_type(input: &str) -> IResult<&str, BloodType> {
    if let Ok((tail, _)) = tag::<_, _, nom::error::Error<_>>("blue")(input) {
        return Ok((tail, BloodType::Blue));
    }
    if let Ok((tail, _)) = tag::<_, _, nom::error::Error<_>>("green")(input) {
        return Ok((tail, BloodType::Green));
    }
    if let Ok((tail, _)) = tag::<_, _, nom::error::Error<_>>("yellow")(input) {
        return Ok((tail, BloodType::Yellow));
    }
    core::result::Result::Err(nom::Err::Error(nom::error::Error::new(
        "",
        nom::error::ErrorKind::Tag,
    )))
}

fn parse_blood_type(input: &str) -> IResult<&str, BloodType> {
    delimited(
        tag("<blood-type>"),
        parse_a_blood_type,
        tag("</blood-type>"),
    )(input)
}

fn parse_level(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("<level>"), digit1, tag("</level>")),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_base_speed(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("<base-speed>"), digit1, tag("</base-speed>")),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_base_strength(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("<base-strength>"), digit1, tag("</base-strength>")),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_base_armor(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("<base-armor>"), digit1, tag("</base-armor>")),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_base_health(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("<base-health>"), digit1, tag("</base-health>")),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_max_speed(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("<max-speed>"), digit1, tag("</max-speed>")),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_max_strength(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("<max-strength>"), digit1, tag("</max-strength>")),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_max_armor(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("<max-armor>"), digit1, tag("</max-armor>")),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_max_health(input: &str) -> IResult<&str, u8> {
    map_res(
        delimited(tag("<max-health>"), digit1, tag("</max-health>")),
        |s: &str| s.parse::<u8>(),
    )(input)
}

fn parse_a_buff_formula<'a, 'b>(input: &'a str, base: &'b str) -> IResult<&'a str, i32> {
    let start_tag: &str = &("<buff-formula-".to_owned() + base + ">");

    let end_tag: &str = &("</buff-formula-".to_owned() + base + ">");

    let bleep = map_res(
        delimited(tag(start_tag), digit1, tag(end_tag)),
        |s: &str| s.parse::<i32>(),
    )(input);
    bleep
}

fn parse_buff_formula<'a, 'b>(input: &'a str, base: &'b str) -> IResult<&'a str, Vec<i32>> {
    let mut output = Vec::<i32>::new();
    let (mut input, first) = parse_a_buff_formula(input, base)?;
    output.push(first);

    loop {
        let temp;
        match multispace0::<_, nom::error::Error<_>>(input) {
            Ok((tail, _)) => {
                temp = tail;
            }
            _ => {
                return Ok((input, output));
            }
        }
        match parse_a_buff_formula(temp, base) {
            Ok((tail, item)) => {
                input = tail;
                output.push(item);
            }
            _ => {
                return Ok((input, output));
            }
        }
    }
}

fn parse_asset_flag(input: &str) -> IResult<&str, u8> {
  map_res(
      delimited(tag("<asset-flag>"), digit1, tag("</asset-flag>")),
      |s: &str| s.parse::<u8>(),
  )(input)
}

fn parse_swarm_value(input: &str) -> IResult<&str, i32> {
  map_res(
      delimited(tag("<pv>"), digit1, tag("</pv>")),
      |s: &str| s.parse::<i32>(),
  )(input)
}

fn parse_attacks(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let mut output = Vec::<(&str, &str)>::new();
    let (mut input, first) = parse_attack(input)?;
    output.push(first);

    loop {
        let temp;
        match multispace0::<_, nom::error::Error<_>>(input) {
            Ok((tail, _)) => {
                temp = tail;
            }
            _ => {
                return Ok((input, output));
            }
        }
        match parse_attack(temp) {
            Ok((tail, item)) => {
                input = tail;
                output.push(item);
            }
            _ => {
                return Ok((input, output));
            }
        }
    }
}
/*    separated_list0(
        multispace1,
        parse_attack,
    )(input)
}*/

fn parse_attack(input: &str) -> IResult<&str, (&str, &str)> {
  let (remainder, id) = delimited(
    tag(r#"<attack id=""#),
    take_till(|c| c == '"'),
    tag(r#"">"#),
  )(input)?;
  let (remainder, body) =take_until("</attack>")(remainder)?;
  let (tail, _ ) = tag("</attack>")(remainder)?;
  Ok((tail, (id, body)))
}

#[allow(dead_code)]
fn get_data() -> &'static str {
    r#"<virmon>

    <asset-type-id>79</asset-type-id>

    <asset-type-version>0</asset-type-version>

    <name>Hyper Ripper 1.0</name>

    <collector-id>14-Hyper Ripper 1.0</collector-id>

    <wave>2</wave>

    <rarity>common</rarity>

    <faction>Magnamod</faction>

    <blood-type>yellow</blood-type>

    <level>2</level>

    <base-speed>15</base-speed>

    <base-strength>110</base-strength>

    <base-armor>5</base-armor>

    <base-health>105</base-health>

    <max-speed>19</max-speed>

    <max-strength>123</max-strength>

    <max-armor>8</max-armor>

    <max-health>121</max-health>

    <buff-formula-speed>6850</buff-formula-speed>

    <buff-formula-speed>7300</buff-formula-speed>

    <buff-formula-speed>7700</buff-formula-speed>

    <buff-formula-speed>8150</buff-formula-speed>

    <buff-formula-strength>53100</buff-formula-strength>

    <buff-formula-strength>53600</buff-formula-strength>

    <buff-formula-strength>54050</buff-formula-strength>

    <buff-formula-strength>54550</buff-formula-strength>

    <buff-formula-strength>55050</buff-formula-strength>

    <buff-formula-strength>55500</buff-formula-strength>

    <buff-formula-strength>56000</buff-formula-strength>

    <buff-formula-strength>56450</buff-formula-strength>

    <buff-formula-strength>56950</buff-formula-strength>

    <buff-formula-strength>57400</buff-formula-strength>

    <buff-formula-strength>57900</buff-formula-strength>

    <buff-formula-strength>58400</buff-formula-strength>

    <buff-formula-strength>58850</buff-formula-strength>

    <buff-formula-armor>66150</buff-formula-armor>

    <buff-formula-armor>77200</buff-formula-armor>

    <buff-formula-armor>88250</buff-formula-armor>

    <buff-formula-health>32750</buff-formula-health>

    <buff-formula-health>33050</buff-formula-health>

    <buff-formula-health>33350</buff-formula-health>

    <buff-formula-health>33650</buff-formula-health>

    <buff-formula-health>33950</buff-formula-health>

    <buff-formula-health>34300</buff-formula-health>

    <buff-formula-health>34600</buff-formula-health>

    <buff-formula-health>34900</buff-formula-health>

    <buff-formula-health>35200</buff-formula-health>

    <buff-formula-health>35500</buff-formula-health>

    <buff-formula-health>35800</buff-formula-health>

    <buff-formula-health>36150</buff-formula-health>

    <buff-formula-health>36450</buff-formula-health>

    <buff-formula-health>36750</buff-formula-health>

    <buff-formula-health>37050</buff-formula-health>

    <buff-formula-health>37350</buff-formula-health>

    <asset-flag>0</asset-flag>

    <pv>180</pv>

    <attack id="165">

      <attack-name>Fireplow</attack-name>

      <type>Melee</type>

      <short-in-combat-description>A fire damage attack</short-in-combat-description>

      <hh-description>A fire damage attack</hh-description>

      <pc-description>A fire damage attack.</pc-description>

      <override-description/>

      <hack-description/>

      <force-damage>Normal</force-damage>

      <keyword-1>Fire</keyword-1>

      <keyword-2>Close</keyword-2>

      <cost>2</cost>

      <ammo>0</ammo>

      <attack-statement id="11">

        <attack-statement-name>40 DAM</attack-statement-name>

        <type>Attack</type>

        <target>Enemy active Virmon</target>

        <stat>Health</stat>

        <verb>-</verb>

        <delay>0</delay>

        <duration>None</duration>

        <repetition>None</repetition>

        <base-damage-formula>40 * ( $MyStrength / 100 )</base-damage-formula>

        <statement-formula>$StatementBaseDamage - $EnemyArmor</statement-formula>

      </attack-statement>

    </attack>

    <attack id="166">

      <attack-name>Hardshell</attack-name>

      <type>Melee</type>

      <short-in-combat-description>This Nanovor places a +5 ARM Override</short-in-combat-description>

      <hh-description>This Nanovor places a +5 Armor Override</hh-description>

      <pc-description>This Nanovor places a +5 Armor Override</pc-description>

      <override-description>+5 ARM</override-description>

      <hack-description/>

      <force-damage>None</force-damage>

      <keyword-1>Override</keyword-1>

      <keyword-2>Self</keyword-2>

      <cost>3</cost>

      <ammo>0</ammo>

      <attack-statement id="21">

        <attack-statement-name>ARM +5 (OR)</attack-statement-name>

        <type>Override</type>

        <target>My active Virmon</target>

        <stat>Armor</stat>

        <verb>+</verb>

        <delay>0</delay>

        <duration>None</duration>

        <repetition>None</repetition>

        <base-damage-formula>0</base-damage-formula>

        <statement-formula>5</statement-formula>

      </attack-statement>

    </attack>

    <attack id="144">

      <attack-name>Red Spike</attack-name>

      <type>Melee</type>

      <short-in-combat-description>Can make Red Spiked attack</short-in-combat-description>

      <hh-description>This Nanovor places an Override  that allows swarm to make Red Spiked attack</hh-description>

      <pc-description>This Nanovor places an Override that allows your swarm to make a Red Spiked attack.</pc-description>

      <override-description>Can make Red Spiked attack</override-description>

      <hack-description/>

      <force-damage>None</force-damage>

      <keyword-1>Override</keyword-1>

      <keyword-2>Self</keyword-2>

      <cost>1</cost>

      <ammo>0</ammo>

      <attack-statement id="135">

        <attack-statement-name>Spike Red (OR)</attack-statement-name>

        <type>Override</type>

        <target>My active Virmon</target>

        <stat>RedSpike</stat>

        <verb>=</verb>

        <delay>0</delay>

        <duration>None</duration>

        <repetition>None</repetition>

        <base-damage-formula/>

        <statement-formula>1</statement-formula>

      </attack-statement>

    </attack>

  </virmon>

  "#; r#"<virmon>

    <asset-type-id>142</asset-type-id>

    <asset-type-version>0</asset-type-version>

    <name>Battle Kraken 1.1</name>

    <collector-id>12-Battle Kraken 1.1</collector-id>

    <wave>1</wave>

    <rarity>rare</rarity>

    <faction>Hexite</faction>

    <blood-type>green</blood-type>

    <level>1</level>

    <base-speed>63</base-speed>

    <base-strength>110</base-strength>

    <base-armor>5</base-armor>

    <base-health>115</base-health>

    <max-speed>63</max-speed>

    <max-strength>110</max-strength>

    <max-armor>5</max-armor>

    <max-health>115</max-health>

    <asset-flag>0</asset-flag>

    <pv>175</pv>

    <attack id="31">

      <attack-name>Crunch</attack-name>

      <type>Melee</type>

      <short-in-combat-description>A damage attack</short-in-combat-description>

      <hh-description>A damage attack</hh-description>

      <pc-description>A damage attack.</pc-description>

      <override-description/>

      <hack-description/>

      <force-damage>Normal</force-damage>

      <keyword-1>Standard</keyword-1>

      <keyword-2>Close</keyword-2>

      <cost>2</cost>

      <ammo>0</ammo>

      <attack-statement id="11">

        <attack-statement-name>40 DAM</attack-statement-name>

        <type>Attack</type>

        <target>Enemy active Virmon</target>

        <stat>Health</stat>

        <verb>-</verb>

        <delay>0</delay>

        <duration>None</duration>

        <repetition>None</repetition>

        <base-damage-formula>40 * ( $MyStrength / 100 )</base-damage-formula>

        <statement-formula>$StatementBaseDamage - $EnemyArmor</statement-formula>

      </attack-statement>

    </attack>

    <attack id="32">

      <attack-name>Tangler</attack-name>

      <type>Melee</type>

      <short-in-combat-description>Target cannot swap this round or the next.</short-in-combat-description>

      <hh-description>Target cannot swap this round or the next.</hh-description>

      <pc-description>An attack that swap-blocks the opponent for this and the next round.</pc-description>

      <override-description/>

      <hack-description>Swap Block  x 2 rd</hack-description>

      <force-damage>Light</force-damage>

      <keyword-1>Standard</keyword-1>

      <keyword-2>Close</keyword-2>

      <cost>1</cost>

      <ammo>0</ammo>

      <attack-statement id="180">

        <attack-statement-name>Blockade (1R, 100%)</attack-statement-name>

        <type>Hack</type>

        <target>Enemy active Virmon</target>

        <stat>Swap %</stat>

        <verb>=</verb>

        <delay>0</delay>

        <duration>1 active round</duration>

        <repetition>None</repetition>

        <base-damage-formula/>

        <statement-formula>0</statement-formula>

      </attack-statement>

    </attack>

    <attack id="151">

      <attack-name>Blue Spike</attack-name>

      <type>Melee</type>

      <short-in-combat-description>Can make Blue Spiked attack</short-in-combat-description>

      <hh-description>This Nanovor places an Override that allowing your swarm to make Blue Spiked attack</hh-description>

      <pc-description>This Nanovor places an Override allowing your swarm to make a Blue Spiked attack.</pc-description>

      <override-description>Can make Blue Spiked attack</override-description>

      <hack-description/>

      <force-damage>None</force-damage>

      <keyword-1>Override</keyword-1>

      <keyword-2>Self</keyword-2>

      <cost>1</cost>

      <ammo>0</ammo>

      <attack-statement id="138">

        <attack-statement-name>Spike Blue (OR)</attack-statement-name>

        <type>Override</type>

        <target>My active Virmon</target>

        <stat>BlueSpike</stat>

        <verb>=</verb>

        <delay>0</delay>

        <duration>None</duration>

        <repetition>None</repetition>

        <base-damage-formula/>

        <statement-formula>1</statement-formula>

      </attack-statement>

    </attack>

  </virmon>"#
}
