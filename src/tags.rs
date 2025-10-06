use std::collections::HashSet;

macro_rules! meme_tags {
    ($($name:ident = ( $base:expr $(, $inherit:ident)* ),)+ $(,)?) => {
        pub(crate) struct MemeTags;

        #[allow(dead_code)]
        impl MemeTags {
            $(
                pub fn $name() -> HashSet<String> {
                    #[allow(unused_mut)]
                    let mut set = $base.iter().map(|&tag| tag.to_string()).collect::<HashSet<_>>();
                    $(
                        set.extend(Self::$inherit());
                    )*
                    set
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! union_tags {
    ($base:expr $(, $inherit:expr)* $(,)?) => {
        {
            #[allow(unused_mut)]
            let mut set = $base;
            $(
                set.extend($inherit);
            )*
            set
        }
    };
}

meme_tags!(
    // 米家游戏
    mihoyo = (["米哈游"]),
    genshin = (["原神"], mihoyo),
    star_rail = (["崩坏：星穹铁道"], mihoyo),
    honkai3 = (["崩坏3"], mihoyo),
    nahida = (["纳西妲", "草神"], genshin),
    hutao = (["胡桃"], genshin),
    klee = (["可莉"], genshin),
    keqing = (["刻晴"], genshin),
    zhongli = (["钟离"], genshin),
    nilou = (["妮露"], genshin),
    yae_miko = (["八重神子"], genshin),
    ayaka = (["神里绫华"], genshin),
    furina = (["芙芙", "芙宁娜"], genshin),
    lynette = (["琳妮特"], genshin),
    elysia = (["爱莉希雅"], genshin),
    bronya = (["布洛妮娅·扎伊切克"], honkai3),
    captain = (["休伯利安号", "舰长"], honkai3),
    griseo = (["格蕾修"], honkai3),
    firefly = (["流萤"], star_rail),
    // 蔚蓝档案
    blue_archive = (["蔚蓝档案", "碧蓝档案"]),
    arisu = (["天童爱丽丝"], blue_archive),
    izuna = (["久田泉奈"], blue_archive),
    key = (["key"], blue_archive),
    mari = (["伊落玛丽"], blue_archive),
    sena = (["冰室濑名"], blue_archive),
    yuuka = (["早濑优香"], blue_archive),
    shiroko = (["砂狼白子"], blue_archive),
    kokona = (["春原心奈", "春原心菜"], blue_archive),
    plana = (["普拉娜"], blue_archive),
    arona = (["阿罗娜"], blue_archive),
    // 明日方舟
    arknights = (["明日方舟"]),
    // 鸣潮
    wuthering_waves = (["鸣潮"]),
    rover = (["漂泊者"], wuthering_waves),
    jinhsi = (["今汐"], wuthering_waves),
    cartethyia = (["卡提希娅"], wuthering_waves),
    verina = (["维里奈"], wuthering_waves),
    phoebe = (["菲比"], wuthering_waves),
    carlotta = (["珂莱塔"], wuthering_waves),
    zhezhi = (["折枝"], wuthering_waves),
    lingyang = (["凌阳"], wuthering_waves),
    mortefi = (["莫特斐"], wuthering_waves),
    // 公主连结
    re_dive = (["公主连结"]),
    karyl = (["凯露"], re_dive),
    // 间谍过家家
    spy_family = (["间谍过家家"]),
    anya = (["阿尼亚·福杰"], spy_family),
    // 孤独摇滚
    bocchi_the_rock = (["孤独摇滚"]),
    bocchi = (["后藤一里", "波奇酱"], bocchi_the_rock),
    nijika = (["伊地知虹夏"], bocchi_the_rock),
    // 咒术回战
    jujutsu_kaisen = (["咒术回战"]),
    sukuna = (["两面宿傩"], jujutsu_kaisen),
    // 葬送的芙莉莲
    sousou_no_frieren = (["葬送的芙莉莲"]),
    frieren = (["芙莉莲"], sousou_no_frieren),
    // 我推的孩子
    oshi_no_ko = (["我推的孩子"]),
    // LoveLive!Superstar!!
    lovelive_superstar = (["LoveLive!Superstar!!"]),
    tan_kuku = (["唐可可"], lovelive_superstar),
    // 莉可丽丝
    lycoris_recoil = (["莉可丽丝"]),
    takina = (["井之上泷奈"], lycoris_recoil),
    walnut = (["胡桃"], lycoris_recoil),
    // 别当欧尼酱了
    onimai = (["别当欧尼酱了"]),
    mahiro = (["绪山真寻"], onimai),
    // 幸运星
    lucky_star = (["幸运星"]),
    konata = (["泉此方"], lucky_star),
    // 凉宫春日
    haruhi = (["凉宫春日"]),
    // 猫和老鼠
    tom_and_jerry = (["猫和老鼠"]),
    tom = (["汤姆"], tom_and_jerry),
    jerry = (["杰瑞"], tom_and_jerry),
    // 瑞克和莫蒂
    rick_and_morty = (["瑞克和莫蒂"]),
    rick = (["瑞克·桑切斯"], rick_and_morty),
    // 东方Project
    touhou = (["东方Project"]),
    // 学园偶像大师
    gakuen_imas = (["学园偶像大师"]),
    kotone = (["藤田琴音"], gakuen_imas),
    // 哈利·波特
    harry_potter = (["哈利·波特"]),
    // VTuber
    gura = (["噶呜·古拉", "Gawr Gura", "鲨鲨"]),
    // VOCALOID
    miku = (["初音未来"]),
    luotianyi = (["洛天依"]),
    // 世界计划
    project_sekai = (["世界计划"]),
    // 魔女的夜宴
    yuzu_soft = (["柚子社"]),
    sabbat_of_the_witch = (["魔女的夜宴"], yuzu_soft),
    ayachi = (["绫地宁宁"], sabbat_of_the_witch),
    murasame = (["丛雨"], sabbat_of_the_witch),
    mako = (["常陆茉子", "茉子", "常陸茉子"], yuzu_soft),
    // 其他
    atri = (["亚托莉", "ATRI", "萝卜子"]),
    capoo = (["猫猫虫", "咖波"]),
    kirby = (["星之卡比"]),
    maimai = (["舞萌"]),
    nekoha = (["猫羽雫"]),
    persona5 = (["女神异闻录5"]),
    stickman = (["火柴人"]),
    sakiko_togawa = (["丰川祥子", "祥子", "豊川祥子"]),
    // 杯子系列
    fleshlight = (["杯子"]),
    fleshlight_air_play = (["空气玩法"], fleshlight),
    fleshlight_cleaning_liquid = (["清洁液"], fleshlight),
    fleshlight_angel = (["天使心"], fleshlight),
    fleshlight_miyuko_kamimiya = (["神宫美优子"], fleshlight),
    fleshlight_purple_spirit = (["紫域精灵"], fleshlight),
    fleshlight_commemorative_edition_saint_sister = (["纪念版圣修女"], fleshlight),
    fleshlight_hoshino_alice = (["拉拉队偶像"], fleshlight),
    fleshlight_idol_heartbeat = (["偶像心跳"], fleshlight),
    fleshlight_jissbon = (["杰士邦"], fleshlight),
    fleshlight_limited_edition_saint_sister = (["限定版版圣修女"], fleshlight),
    fleshlight_liuli_zi = (["琉璃子"], fleshlight),
    fleshlight_saint_sister = (["圣修女"], fleshlight),
    fleshlight_selena = (["魔女之森"], fleshlight),
    fleshlight_pure_buttocks = (["纯洁臀"], fleshlight),
    fleshlight_summer_liuli_zi = (["夏日琉璃子"], fleshlight),
    fleshlight_taimanin_asgi = (["对魔忍"], fleshlight),
    fleshlight_qiaobenyouxi = (["桥本友希"], fleshlight),
    fleshlight_xingnai = (["杏奈"], fleshlight),
    fleshlight_starter_pack = (["新手礼包"], fleshlight),
);
