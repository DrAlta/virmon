#![allow(unused_imports)]
use std::collections::HashMap;

use nom::{
    IResult,
    character::complete::{digit1, multispace0, multispace1},
    bytes::complete::{take_till, take_until, tag},
    sequence::{delimited, tuple},
};
use nom::combinator::map_res;
use nom::multi::separated_list0;

trait AttackUsedBy<'c> {
  fn add_used_by(& mut self, key: &'c str, thingie: &'c str);
}
impl<'b> AttackUsedBy<'b> for HashMap::<&'b str, Vec<&'b str>> {

  fn add_used_by(& mut self, key: &'b str, value: &'b str) {
    if let Some(x) = self.get_mut(&key) {
      x.push(value);
    } else {
      self.insert(key, vec![value]);
    }
  }
}


#[allow(dead_code)]
#[derive(Debug)]
struct Virmon<'a>{
    asset_type_id: u8, 
    asset_type_version: u8,
    name: &'a str,
}
impl<'a> Virmon<'a> {
    fn new(asset_type_id: u8, asset_type_version: u8, name: &'a str) -> Self {
        Virmon{asset_type_id, asset_type_version, name}
    }
}
#[allow(dead_code)]
fn print_attacks(attack_descs: HashMap::<&str, &str>, attack_used_by: HashMap::<&str, Vec::<&str>>) {
    for (id, _attack) in &attack_descs {
        println!(
            "{} used by {:?}", 
            id, 
            attack_used_by.get(id).unwrap()
        );
    }
}
fn main() {
    println!("ass id:{:?}", parse_asset_type_id(r#"<asset-type-id>79</asset-type-id>"#));
    println!("ass ver:{:?}", parse_asset_type_version(r#"<asset-type-version>0</asset-type-version>"#));


    println!("attack:{:?}", parse_attacks(r#"<attack>AttackFoo</attack>

<attack>AttackBar</attack>"#)) ;



    let unprocessed = "  foo";

    let (unprocessed, _) = multispace0::<_, nom::error::Error<_>>(unprocessed).unwrap();

    let nanovors = Vec::<Virmon>::new();
    let _attack_descs = HashMap::<&str, &str>::new();
    let mut attack_used_by = HashMap::<&str, Vec<&str>>::new();
    loop {
        let Ok((unprocessed, (nanovor, hys_attacks))) = parse_virmon(unprocessed) else {break};
        println!("{:?}", nanovor);
        for attack in &hys_attacks {
            let (_, attack_id) = parse_attack_id(attack).unwrap();
            attack_used_by.add_used_by(attack_id, nanovor.name);
        }
    println!("{}", unprocessed);
    }
    if false {
      println!("{:?}", nanovors);
    }
}


fn parse_attack_id(input: &str) -> IResult<&str, &str>{
    delimited(
        tag(r#"<attack id=""#),
        take_till(|c| c == '"'),
        tag(r#"">"#)
    )(input)
}


fn parse_virmon(input: &str) -> IResult<&str, (Virmon, Vec<&str>)>{


    let (mut input, _) = tag("<virmon>")(input)?;
    (input, _) = multispace0(input)?;
    let (mut input, asset_type_id) = parse_asset_type_id(input)?;
    (input, _) = multispace0(input)?;
    let ( input, asset_type_version) = parse_asset_type_version(input)?;

    let (mut input, name) = parse_name(input)?;

    (input, _) = multispace0(input)?;
    let (mut input, attacks) = parse_attacks(input)?;
    (input, _) = multispace0(input)?;
    let my_virmon = Virmon::new(asset_type_id, asset_type_version, name);
    let (_, tail) = tag("</virmon>")(input)?;
    Ok((tail, (my_virmon, attacks)))

}

fn parse_asset_type_id(input: &str) -> IResult<&str, u8>{
    map_res(
        delimited(
            tag("<asset-type-id>"),
            digit1,
            tag("/<asset-type-id>")
        ),
        |s: &str| s.parse::<u8>()
    )(input)
}

fn parse_asset_type_version(input: &str) -> IResult<&str, u8>{
    map_res(
        delimited(
           tag("<asset-type-version>"),
            digit1,
            tag("</asset-type-version>")
        ),
        |s: &str| s.parse::<u8>()
    )(input)
}


fn parse_name(input: &str) -> IResult<&str, &str>{
    delimited(
        tag("<name>"),
        take_till(|c| c == '<'),
        tag("</name>")
    )(input)
}


fn parse_attacks(input: &str)-> IResult<&str, Vec<&str>> {
    separated_list0(
        multispace1, 
        parse_attack,
    )(input)
}


fn parse_attack(input: &str) -> IResult<&str, &str>{
    delimited(
        tag("<attack>"),
        take_until("</attack>"),
        tag("</attack>")
    )(input)
}
#[allow(dead_code)]
fn get_data() -> &'static str {
    r#"  <virmon>

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

  <virmon>

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