mod buddy;
mod bayonetta;
mod bayonetta_bulletarts;

mod captain;
mod chrom;
mod cloud;
pub mod common;

mod daisy;
mod dedede;
mod demon;
mod diddy;
mod dolly;
mod donkey;
mod duckhunt;

mod edge;
mod eflame;
mod elight;

mod falco;
mod fox;

mod ganon;
mod gaogaen;
mod gamewatch;
mod gekkouga;

mod hero;

mod ike;
mod inkling;

mod jack;

mod kamui;
mod ken;
mod kirby;
mod koopa;
mod koopajr;
mod krool;

mod link;
mod littlemac;
mod lucario;
mod lucas;
mod lucina;
mod luigi;

mod mario;
mod mariod;
mod marth;
mod master;
mod metaknight;
mod mewtwo;

mod nana;
mod ness;

mod packun;
mod pacman;
mod palutena;
mod peach;
mod pfushigisou;
mod pichu;
mod pickel;
mod pikachu;
mod pikmin;
mod pit;
mod pitb;
mod plizardon;
mod popo;
mod purin;
mod pzenigame;

mod reflet;
mod richter;
mod ridley;
mod robot;
mod rockman;
mod rosetta;
mod roy;
mod ryu;

mod samus;
mod samusd;
mod sheik;
mod shizue;
mod shulk;
mod simon;
mod snake;
mod sonic;
mod szerosuit;

mod tantan;
mod tink;
mod trail;

mod wario;
mod wiifit;
mod wolf;

mod yoshi;
mod younglink;

mod zelda;

pub fn install(){
    bayonetta::install();   //okay
    bayonetta_bulletarts::install(); //no hitboxes
    buddy::install();       //testing
    captain::install();     //testing
    chrom::install();       //okay
    cloud::install();       //testing
    common::install();     //lol
    daisy::install();       //testing
    dedede::install();      //testing
    demon::install();       //testing, tons of new stuff
    diddy::install();       //testing
    dolly::install();       //testing
    donkey::install();      //testing
    duckhunt::install();    //testing
    edge::install();        //testing
    eflame::install();      //testing
    elight::install();      //testing
    falco::install();       //testing
    fox::install();         //testing
    ganon::install();       //okay
    gaogaen::install();     //testing
    gamewatch::install();
    gekkouga::install();
    hero::install();        //testing
    ike::install();         //testing
    inkling::install();     //ink arts don't work as intended. wrong angle and doesn't require ink
    jack::install();        //testing
    kamui::install();       //okay, ryusensya might have to much stun
    ken::install();         //testing
    kirby::install();       //okay
    koopa::install();       //testing
    koopajr::install();     //testing
    krool::install();       //okay
    link::install();        //testing
    littlemac::install();   //testing
    lucario::install();     //testing
    lucas::install();       //testing
    lucina::install();      //okay
    luigi::install();       //testing
    mario::install();       //testing
    mariod::install();      //testing
    marth::install();       //okay
    master::install();      //okay
    metaknight::install();  //testing
    mewtwo::install();      //testing
    nana::install();
    ness::install();        //testing
    packun::install();
    pacman::install();
    palutena::install();    //testing
    peach::install();       //testing
    pfushigisou::install();
    pichu::install();       //testing
    pickel::install();
    pikachu::install();
    pikmin::install();
    pit::install();         //testing
    pitb::install();        //testing
    plizardon::install();
    popo::install();
    purin::install();       //testing
    pzenigame::install();
    reflet::install();      //testing
    richter::install();     //testing
    ridley::install();
    robot::install();
    rockman::install();
    rosetta::install();     //testing
    roy::install();         //testing
    ryu::install();         //testing
    samus::install();       //testing
    samusd::install();      //testing
    sheik::install();       //testing
    shizue::install();      //testing
    shulk::install();       //testing
    simon::install();       //testing
    snake::install();       //testing
    sonic::install();
    szerosuit::install();   //testing
    tantan::install();      //testing
    tink::install();        //testing
    trail::install();       //testing
    wario::install();
    wiifit::install();      //testing
    wolf::install();        //testing
    yoshi::install();       //testing
    younglink::install();
    zelda::install();       //testing
}