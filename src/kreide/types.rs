use std::ffi::c_void;
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MMNDIEBMDNL {
	pub native_object: NativeObject,
	pub HECCDOHIAFD: *const SkillCharacterComponent,	// 0x10
	pub GNBEIGMFGIP: *const c_void,	// 0x18
	pub HMCDHMFHABF: OLHMAHMMBNN,	// 0x20
	pub FIMNOPAAFEP: *const TurnBasedAbilityComponent,	// 0x78
	pub MKMMNLODHDD: *const c_void,	// 0x80
	pub NMJEMHAMIHD: i32,	// 0x88
	pub DADCNHAIOMI: i32,	// 0x8c
	pub OOIFIGDBNBO: i32	// 0x90
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OLHMAHMMBNN {
	pub JBHFMCDFPPL: *const c_void,	// 0x0
	pub FKHHOBBFMEH: *const NativeString,	// 0x8
	pub BAICECGKLBG: *const c_void,	// 0x10
	pub OAAMONICNLE: *const NativeArray<NativeObject>,	// 0x18
	pub MOIPJLBAODO: i32,	// 0x20
	pub NMJEMHAMIHD: i32,	// 0x24
	pub AHNHNPOCNDJ: bool,	// 0x28
	pub OBNPIDPHFDE: bool,	// 0x29
	pub EKFIDPFOILC: bool,	// 0x2a
	pub NMKBJGEONOJ: bool,	// 0x2b
	pub EDIDAHIELAG: *const c_void,	// 0x30
	pub OKHBBILFBND: [u8; 0x2],	// 0x38
	pub LDJAAEOOOLC: [u8; 0x2],	// 0x3a
	pub MHFEBJINMBP: bool,	// 0x3c
	pub AJENNABILJC: bool,	// 0x3d
	pub GJIMBAPCJLF: bool,	// 0x3e
	pub ODNBNHFLMCD: *const c_void,	// 0x40
	pub FGJEHAKCLNL: *const c_void,	// 0x48
	pub KGKBLOJMDPH: bool	// 0x50
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NOPBAAAGGLA {
	pub native_object: NativeObject,
	pub AAHMMHBHMFN: [u8; 0x90],	// 0x10
	pub LGGEDDMACDF: *const NativeString,	// 0xa0
	pub MDEHKOOKJCK: *const NativeArray<NativeObject>,	// 0xa8
	pub PBHCGDFPEED: *const c_void,	// 0xb0
	pub HKFGOHGKOGK: *const c_void,	// 0xb8
	pub JKCOIOLCMEP: *const TurnBasedAbilityComponent,	// 0xc0
	pub KNDJNKNHFFG: *const TurnBasedAbilityComponent,	// 0xc8
	pub JODAJBNCCNP: *const NativeArray<NativeObject>,	// 0xd0
	pub BEAJGANIDLJ: *const c_void,	// 0xd8
	pub FKKDFMPMJHG: *const NativeArray<NativeObject>,	// 0xe0
	pub APDLLHIMMEM: FixPoint,	// 0xe8
	pub CAILJEGIDKL: FixPoint,	// 0xf0
	pub FLMEBELNIKK: FixPoint,	// 0xf8
	pub FGIPOLJPICM: FixPoint,	// 0x100
	pub NAGMKEABGEE: FixPoint,	// 0x108
	pub PJPKDAKBEJI: FixPoint,	// 0x110
	pub BEGDMOGLLGM: FixPoint,	// 0x118
	pub BLFCEOMPDKK: FixPoint,	// 0x120
	pub DBBDIMCJIKE: FixPoint,	// 0x128
	pub KMIKODLPNGL: i32,	// 0x130
	pub OJGNIBKADHK: u32,	// 0x134
	pub DCEBGGFOFAO: FixPoint,	// 0x138
	pub BGBOFNMKDNJ: FixPoint,	// 0x140
	pub CGMHNNNOKAI: FixPoint,	// 0x148
	pub JFMADBFKBDK: FixPoint,	// 0x150
	pub MKNDMBOCCBO: FixPoint,	// 0x158
	pub LJGPDLDGCEO: FixPoint,	// 0x160
	pub GHBPOPKEGLE: FixPoint,	// 0x168
	pub BBNMJNPDOCP: FixPoint,	// 0x170
	pub ODBPMMGBKGA: FixPoint,	// 0x178
	pub PJNEJPNBNMP: FixPoint,	// 0x180
	pub GLGFEKEMMJJ: FixPoint,	// 0x188
	pub EFAAJEAENFF: FixPoint,	// 0x190
	pub KOCOLHHLFLD: [u8; 0x40],	// 0x198
	pub DJHDAOOEJOF: FixPoint,	// 0x1d8
	pub GCFCCDPIACO: FixPoint,	// 0x1e0
	pub EBDJHPNOALL: FixPoint,	// 0x1e8
	pub EBDJIHNKAOC: FixPoint,	// 0x1f0
	pub FFFOLNDHIEH: [u8; 0x48],	// 0x1f8
	pub HCGBHCPHDKJ: FixPoint,	// 0x240
	pub KODEDHBLGGH: FixPoint,	// 0x248
	pub MGFECPHDPHB: FixPoint,	// 0x250
	pub DINCHAHPEAC: FixPoint,	// 0x258
	pub MKIMEBNOEGI: FixPoint,	// 0x260
	pub KLMAGCLFBAO: FixPoint,	// 0x268
	pub KOEGLFLGADD: FixPoint,	// 0x270
	pub MNGPDEOEHPE: FixPoint,	// 0x278
	pub JHOHCEFOJNB: FixPoint,	// 0x280
	pub EFFODBPOOCN: FixPoint,	// 0x288
	pub COIDNPMCCFG: FixPoint,	// 0x290
	pub MAKENPDPHDN: FixPoint,	// 0x298
	pub COKMLMJPKLH: u32,	// 0x2a0
	pub POLANGDKOKH: FixPoint,	// 0x2a8
	pub HEMFDDDJOGK: bool,	// 0x2b0
	pub GFFCEBJGABG: bool,	// 0x2b1
	pub HKNLHAMMIIM: bool,	// 0x2b2
	pub FNBALMGFGDM: bool,	// 0x2b3
	pub AHHEDGLMDMG: i32,	// 0x2b4
	pub IAAJMHADJDG: FixPoint,	// 0x2b8
	pub CCLFKIPGJOG: FixPoint,	// 0x2c0
	pub DGFBMAPFPNH: FixPoint,	// 0x2c8
	pub GBENLNNEIJM: FixPoint,	// 0x2d0
	pub OEPAPFDLMML: FixPoint,	// 0x2d8
	pub HNJBAFCNNDD: FixPoint,	// 0x2e0
	pub PDCMJAMPJNL: FixPoint,	// 0x2e8
	pub ALOGNJIBIPG: FixPoint,	// 0x2f0
	pub CMNBOEIDAOD: FixPoint,	// 0x2f8
	pub BDGDFKGOLPJ: FixPoint,	// 0x300
	pub EPJEDLOBFPG: FixPoint,	// 0x308
	pub AHOCGHANMCE: FixPoint,	// 0x310
	pub MKMILJKLJON: [u8; 0x58],	// 0x318
	pub JIINJMJGCOH: FixPoint,	// 0x370
	pub CAANBNCPACE: bool,	// 0x378
	pub EKBHFCODKFO: bool,	// 0x379
	pub IJJHMGEHMHB: bool,	// 0x37a
	pub AHPFPMEGEKG: bool,	// 0x37b
	pub GCGEEFLGCIG: i32,	// 0x37c
	pub NCOHIAPKAED: FixPoint,	// 0x380
	pub GAALBDHLFOG: FixPoint,	// 0x388
	pub PJLPGAGKIDE: FixPoint,	// 0x390
	pub PGOHAIPOCNK: FixPoint,	// 0x398
	pub FFCGIMAMDPP: FixPoint,	// 0x3a0
	pub HHEIPBOKCOH: [u8; 0x40],	// 0x3a8
	pub GJNAGCJONAO: FixPoint,	// 0x3e8
	pub HJAEPANAFLN: FixPoint,	// 0x3f0
	pub DEOICHHPAIF: FixPoint,	// 0x3f8
	pub GCNOMMHFPOG: FixPoint,	// 0x400
	pub JFKEEOMKMLI: FixPoint,	// 0x408
	pub MNAPDDFFHJF: bool,	// 0x410
	pub KDCHAHHPPGD: bool,	// 0x411
	pub DPEJKHJPLAC: bool,	// 0x412
	pub BBDANLEJCIA: bool,	// 0x413
	pub CFBOJBAJCEA: i32,	// 0x414
	pub GLPLDJKMOBE: FixPoint,	// 0x418
	pub ANHNDBECCJD: [u8; 0x40],	// 0x420
	pub KPELFJICFDH: FixPoint,	// 0x460
	pub GMBACFCLEGD: FixPoint,	// 0x468
	pub IICNDPJGCFA: i32,	// 0x470
	pub JGHJIGOCPNP: i32,	// 0x474
	pub MJMDGNPPILN: FixPoint,	// 0x478
	pub DBNKBGKCMKH: FixPoint,	// 0x480
	pub CINNHMENLIJ: FixPoint,	// 0x488
	pub JNFPCNAKNOP: FixPoint,	// 0x490
	pub ABIPIIBIIBE: FixPoint,	// 0x498
	pub PNGJIDMHIOE: FixPoint,	// 0x4a0
	pub PAIGBKBOKDI: FixPoint,	// 0x4a8
	pub DPPDEDGCLJJ: FixPoint,	// 0x4b0
	pub HMMMDOHLFEP: FixPoint,	// 0x4b8
	pub ELGMFJLGCPH: FixPoint,	// 0x4c0
	pub FNDCNMHMCIC: FixPoint,	// 0x4c8
	pub BDLFBDLDEND: [u8; 0x48],	// 0x4d0
	pub AMAJNHHAJIM: FixPoint,	// 0x518
	pub FMMBMJKNAHI: FixPoint,	// 0x520
	pub DKOIGIHEBCD: FixPoint,	// 0x528
	pub PGGOANFBJON: FixPoint,	// 0x530
	pub ILNAKPIOOAK: FixPoint,	// 0x538
	pub ENFFBMJBEDP: FixPoint,	// 0x540
	pub ACDFHOGEMCC: [u8; 0x40],	// 0x548
	pub JCPEINMPKAM: FixPoint,	// 0x588
	pub BKIFAEKCIHN: FixPoint,	// 0x590
	pub FOLCDHNIMMI: FixPoint,	// 0x598
	pub NHHNLMOBEGH: FixPoint,	// 0x5a0
	pub GIHPOCDLJOA: FixPoint,	// 0x5a8
	pub APDDLHNGGIM: AttackType,	// 0x5b0
	pub KBKGNDFAKGD: bool,	// 0x5b4
	pub EGINKGPDNPK: bool,	// 0x5b5
	pub JICCOEHBPJJ: bool,	// 0x5b6
	pub EJJMIFKCFHP: bool,	// 0x5b7
	pub JEHMOKDJDDE: FixPoint,	// 0x5b8
	pub MLKFKKACBCE: FixPoint,	// 0x5c0
	pub GOHOJAIMDNM: FixPoint,	// 0x5c8
	pub DJCAFPFKOGP: FixPoint,	// 0x5d0
	pub BJAEJMLMJCL: FixPoint,	// 0x5d8
	pub GNMAKKBFOCH: FixPoint,	// 0x5e0
	pub NEPGNKOMAAA: FixPoint,	// 0x5e8
	pub OHBMMFAFMDP: FixPoint,	// 0x5f0
	pub KDJBABPDHEG: FixPoint,	// 0x5f8
	pub MHEBPGAHFCB: FixPoint	// 0x600
}
pub mod rpg {
	use std::ffi::c_void;
	use crate::kreide::types::*;
	pub mod client {
	use std::ffi::c_void;
	use crate::kreide::types::*;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModuleManager {
	pub native_object: NativeObject,
	pub FarmModule: *const c_void,	// 0x10
	pub ArchiveModule: *const c_void,	// 0x18
	pub ActivitySummonModule: *const c_void,	// 0x20
	pub SwitchHandModule: *const c_void,	// 0x28
	pub TalkModule: *const c_void,	// 0x30
	pub RogueTournModule: *const c_void,	// 0x38
	pub MapPropOverrideConditionModule: *const c_void,	// 0x40
	pub RoleTrialModule: *const c_void,	// 0x48
	pub RogueHandbookModule: *const c_void,	// 0x50
	pub GachaModule: *const c_void,	// 0x58
	pub MusicAlbumModule: *const c_void,	// 0x60
	pub ItemComposeModule: *const c_void,	// 0x68
	pub RaidCollectionModule: *const c_void,	// 0x70
	pub PlayerModule: *const c_void,	// 0x78
	pub ActivityQuestTimeLimitModule: *const c_void,	// 0x80
	pub MarbleModule: *const c_void,	// 0x88
	pub TitanAtlasModule: *const c_void,	// 0x90
	pub HeliobusModule: *const c_void,	// 0x98
	pub BattlePassModule: *const c_void,	// 0xa0
	pub RogueAdventureModule: *const c_void,	// 0xa8
	pub MissionChronicleModule: *const c_void,	// 0xb0
	pub NovelModule: *const c_void,	// 0xb8
	pub EraFlipperModule: *const c_void,	// 0xc0
	pub ActivityMusicRhythmModule: *const c_void,	// 0xc8
	pub RogueModule: *const c_void,	// 0xd0
	pub ChatModule: *const c_void,	// 0xd8
	pub GamePlayLockModule: *const c_void,	// 0xe0
	pub RechargeShopModule: *const c_void,	// 0xe8
	pub AdventureModule: *const c_void,	// 0xf0
	pub NavMapModule: *const c_void,	// 0xf8
	pub ToastQueueModule: *const c_void,	// 0x100
	pub ActivityPlayerReturnModule: *const c_void,	// 0x108
	pub SilverWolfModule: *const c_void,	// 0x110
	pub RogueArcadeModule: *const c_void,	// 0x118
	pub BattleCollegeModule: *const c_void,	// 0x120
	pub CompanionMissionActivityModule: *const c_void,	// 0x128
	pub FormationMoveModule: *const c_void,	// 0x130
	pub ActivityStrongChallengeModule: *const c_void,	// 0x138
	pub EarlyAccessModule: *const c_void,	// 0x140
	pub FiveDimModule: *const c_void,	// 0x148
	pub ActivityBenefitModule: *const c_void,	// 0x150
	pub FantasticStoryActivityModule: *const c_void,	// 0x158
	pub ActivityParkourModule: *const c_void,	// 0x160
	pub ScheduleModule: *const c_void,	// 0x168
	pub PhotoGraphModule: *const c_void,	// 0x170
	pub ActivityFeverTimeModule: *const c_void,	// 0x178
	pub ActivityGuessTheSilhouetteModule: *const c_void,	// 0x180
	pub AlleyModule: *const c_void,	// 0x188
	pub TextJoinModule: *const c_void,	// 0x190
	pub LobbyModule: *const c_void,	// 0x198
	pub SystemOpenModule: *const c_void,	// 0x1a0
	pub ActivityAetherDivideModule: *const c_void,	// 0x1a8
	pub PersonalizeModule: *const c_void,	// 0x1b0
	pub RogueMagicModule: *const c_void,	// 0x1b8
	pub ChessRogueModule: *const c_void,	// 0x1c0
	pub CumulativeConsumptionModule: *const c_void,	// 0x1c8
	pub HandbookModule: *const c_void,	// 0x1d0
	pub GridFightModule: *const c_void,	// 0x1d8
	pub ShopModule: *const c_void,	// 0x1e0
	pub PingPongModule: *const c_void,	// 0x1e8
	pub AnniversaryAvatarDeliverModule: *const c_void,	// 0x1f0
	pub AvatarModule: *const c_void,	// 0x1f8
	pub TravelBrochureModule: *const c_void,	// 0x200
	pub PlanetFesModule: *const c_void,	// 0x208
	pub RollShopModule: *const c_void,	// 0x210
	pub FriendModule: *const c_void,	// 0x218
	pub PayModule: *const c_void,	// 0x220
	pub LuaDataModule: *const c_void,	// 0x228
	pub ShareModule: *const c_void,	// 0x230
	pub modules: *const NativeArray<NativeObject>,	// 0x238
	pub PerformanceRecallModule: *const c_void,	// 0x240
	pub SpaceZooModule: *const c_void,	// 0x248
	pub FightFestModule: *const c_void,	// 0x250
	pub ActivitySwordTrainingModule: *const c_void,	// 0x258
	pub AetherDivideModule: *const c_void,	// 0x260
	pub OfferingModule: *const c_void,	// 0x268
	pub MissionModule: *const c_void,	// 0x270
	pub ColonyCollectionPuzzleModule: *const c_void,	// 0x278
	pub TrainModule: *const c_void,	// 0x280
	pub DrinkMakerModule: *const c_void,	// 0x288
	pub ActivityPhotoExhibitionModule: *const c_void,	// 0x290
	pub ServerPrefsModule: *const c_void,	// 0x298
	pub PunkLordModule: *const c_void,	// 0x2a0
	pub FightActivityModule: *const c_void,	// 0x2a8
	pub MessageModule: *const c_void,	// 0x2b0
	pub GameStateServiceModule: *const c_void,	// 0x2b8
	pub ChallengeModule: *const c_void,	// 0x2c0
	pub FloorConnectivityModule: *const c_void,	// 0x2c8
	pub StoryLineModule: *const c_void,	// 0x2d0
	pub MovieRacingModule: *const c_void,	// 0x2d8
	pub StoryTokenModule: *const c_void,	// 0x2e0
	pub ActivityTelevisionModule: *const c_void,	// 0x2e8
	pub BigMapModule: *const c_void,	// 0x2f0
	pub MultiPathAvatarModule: *const c_void,	// 0x2f8
	pub EvolveBuildModule: *const c_void,	// 0x300
	pub InventoryModule: *const c_void,	// 0x308
	pub EntityTimeRewindModule: *const c_void,	// 0x310
	pub TreasureDungeonModule: *const c_void,	// 0x318
	pub StarFightModule: *const c_void,	// 0x320
	pub CatchGhostModule: *const c_void,	// 0x328
	pub AnniversaryCollectionModule: *const c_void,	// 0x330
	pub MapConnectivityModule: *const c_void,	// 0x338
	pub BattleModule: *const c_void,	// 0x340
	pub BattleTipsModule: *const c_void,	// 0x348
	pub MultiplayerGameModule: *const c_void,	// 0x350
	pub TeamModule: *const c_void,	// 0x358
	pub GrowthModule: *const c_void,	// 0x360
	pub TransferModule: *const c_void,	// 0x368
	pub FindChestModule: *const c_void,	// 0x370
	pub DialogueModule: *const c_void,	// 0x378
	pub RecommendModule: *const c_void,	// 0x380
	pub LoginModule: *const c_void,	// 0x388
	pub _ModuleInitRequestList: *const NativeArray<NativeObject>,	// 0x390
	pub DifficultyAdjustModule: *const c_void,	// 0x398
	pub HeartDialModule: *const c_void,	// 0x3a0
	pub ExpeditionModule: *const c_void,	// 0x3a8
	pub ActivityTrackPhotoModule: *const c_void,	// 0x3b0
	pub PamSkinModule: *const c_void,	// 0x3b8
	pub AntiAddictionModule: *const c_void,	// 0x3c0
	pub BoxingClubModule: *const c_void,	// 0x3c8
	pub ChimeraModule: *const c_void,	// 0x3d0
	pub RelicModule: *const c_void,	// 0x3d8
	pub TutorialSupportModule: *const c_void,	// 0x3e0
	pub MatchThreeModule: *const c_void,	// 0x3e8
	pub MonopolyModule: *const c_void,	// 0x3f0
	pub WorldShop4ThModule: *const c_void,	// 0x3f8
	pub MultiFloorConflictModule: *const c_void,	// 0x400
	pub ActivityClockParkModule: *const c_void,	// 0x408
	pub AchievementModule: *const c_void,	// 0x410
	pub LoadingTipsModule: *const c_void,	// 0x418
	pub ActivityModule: *const c_void,	// 0x420
	pub WolfBroShootingModule: *const c_void,	// 0x428
	pub OperationModule: *const c_void,	// 0x430
	pub PamModule: *const c_void,	// 0x438
	pub QuestModule: *const c_void,	// 0x440
	pub WhiteListInteractUploadModule: *const c_void,	// 0x448
	pub RaidModule: *const c_void,	// 0x450
	pub FeatureSwitchModule: *const c_void,	// 0x458
	pub PetModule: *const c_void,	// 0x460
	pub TarotBookModule: *const c_void,	// 0x468
	pub MaterialSubmissionModule: *const c_void,	// 0x470
	pub MapRotationModule: *const c_void,	// 0x478
	pub MultiPlayerActivityModule: *const c_void,	// 0x480
	pub MultipleDropModule: *const c_void,	// 0x488
	pub BattleEventModule: *const c_void,	// 0x490
	pub EntityScoreModule: *const c_void,	// 0x498
	pub MuseumModule: *const c_void,	// 0x4a0
	pub TrainPartyModule: *const c_void,	// 0x4a8
	pub MissionTimelineModule: *const c_void,	// 0x4b0
	pub isInited: bool	// 0x4b8
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AvatarData {
	pub native_object: NativeObject,
	pub _AvatarRowData: *const c_void,	// 0x10
	pub SpecialRow__BackingField: *const c_void,	// 0x18
	pub AvatarPropertyData__BackingField: *const c_void,	// 0x20
	pub CombatPowerData__BackingField: *const c_void,	// 0x28
	pub PromotedBeforeData__BackingField: *const c_void,	// 0x30
	pub _ExtraPropertyAddition: *const c_void,	// 0x38
	pub _TrialEquipment: *const c_void,	// 0x40
	pub _AvatarName: *const NativeString,	// 0x48
	pub LevelUpedBeforeData__BackingField: *const c_void,	// 0x50
	pub RelicsData__BackingField: *const c_void,	// 0x58
	pub UltraSkillConfig__BackingField: *const c_void,	// 0x60
	pub GrowUpBeforeData__BackingField: *const c_void,	// 0x68
	pub HasTakenPromotionRewardList__BackingField: *const NativeArray<u32>,	// 0x70
	pub Row__BackingField: *const c_void,	// 0x78
	pub ServantData__BackingField: *const AvatarServantData,	// 0x80
	pub SkillTreeData: *const c_void,	// 0x88
	pub _SkillDataMap: *const c_void,	// 0x90
	pub _SkinIDList: *const NativeArray<u32>,	// 0x98
	pub CurrentExp__BackingField: u32,	// 0xa0
	pub _AdventurePlayerID: u32,	// 0xa4
	pub RealID__BackingField: u32,	// 0xa8
	pub _BaseID: u32,	// 0xac
	pub Rank__BackingField: u32,	// 0xb0
	pub DressedSkinID__BackingField: u32,	// 0xb4
	pub IsMarked__BackingField: bool,	// 0xb8
	pub IsNew__BackingField: bool,	// 0xb9
	pub IsDisplayOnly__BackingField: bool,	// 0xba
	pub AvatarType__BackingField: i32,	// 0xbc
	pub Promotion__BackingField: u32,	// 0xc0
	pub Level__BackingField: u32,	// 0xc4
	pub FirstMetTimeStamp: u64,	// 0xc8
	pub SpecialAvatarID__BackingField: u32,	// 0xd0
	pub EquipmentUID__BackingField: u32	// 0xd4
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextID {
	pub hash: i32,	// 0x0
	pub hash64: u64	// 0x8
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AvatarServantData {
	pub native_object: NativeObject,
	pub _SkillDataMap: *const c_void,	// 0x10
	pub _Json: *const c_void,	// 0x18
	pub _ServantRowData: *const c_void,	// 0x20
	pub _Row: *const c_void,	// 0x28
	pub _AvatarData: *const AvatarData	// 0x30
}
	}
	pub mod gamecore {
	use std::ffi::c_void;
	use crate::kreide::types::*;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CharacterDataComponent {
	pub _parent_object:  GameComponentBase,	// 0x0
	pub _DummpyEntityList: *const NativeArray<NativeObject>,	// 0x18
	pub _DynamicScaleAdaptTypes: *const NativeArray<NativeObject>,	// 0x20
	pub _DynamicScaleAdaptConfigs: *const NativeArray<NativeObject>,	// 0x28
	pub HideDisplayInfoSkillNames: *const c_void,	// 0x30
	pub _DynamicScaleAdaptEffectPathRule: *const c_void,	// 0x38
	pub Summoner: *const GameEntity,	// 0x40
	pub _RowData: *const c_void,	// 0x48
	pub _CharacterUICustomValueDict: *const c_void,	// 0x50
	pub JsonConfig__BackingField: *const CharacterConfig,	// 0x58
	pub GridFightTag__BackingField: i32,	// 0x60
	pub CreateReason: i32,	// 0x64
	pub EnhancedState: i32,	// 0x68
	pub LocalOffsetAsMoveTarget__BackingField: [u8; 0xc],	// 0x6c
	pub LineupIndex: i32,	// 0x78
	pub SpawnTurnCount: u32,	// 0x7c
	pub DisableRootYawMapping__BackingField: bool,	// 0x80
	pub TriggerLimbo: bool,	// 0x81
	pub LastActTurnCount__BackingField: u32,	// 0x84
	pub CharacterID__BackingField: u32,	// 0x88
	pub IsVisibleInViewMode__BackingField: bool,	// 0x8c
	pub IsBodyPart: bool,	// 0x8d
	pub DisableHeadLookAtActionEntityOverride: [u8; 0x2],	// 0x8e
	pub _SaveModelWhenDeadOverride: *const c_void	// 0x90
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EntityManager {
	pub native_object: NativeObject,
	pub DataViewUISelectSummonerOfUncreatedServant__BackingField: *const GameEntity,	// 0x10
	pub _EntityUniqueNameDict: *const NativeArray<NativeObject>,	// 0x18
	pub DataViewUISelectFadeOutEntity__BackingField: *const GameEntity,	// 0x20
	pub _AllTeamEntityList: *const NativeArray<GameEntity>,	// 0x28
	pub _UniqueNamedEntityDictionary: *const c_void,	// 0x30
	pub EntityGORoot__BackingField: *const c_void,	// 0x38
	pub _AllEntityDictionary: *const c_void,	// 0x40
	pub LittleGameGORoot__BackingField: *const c_void,	// 0x48
	pub GroupGORoot__BackingField: *const c_void,	// 0x50
	pub _ServerEntityIDToEntityDict: *const c_void,	// 0x58
	pub _ProcessEntityTeamChangeDelg: *const c_void,	// 0x60
	pub _OwnerWorldRef: *const c_void,	// 0x68
	pub _SnapshotEntityMap: *const c_void,	// 0x70
	pub _GroupEntityIDToEntityDict: *const c_void,	// 0x78
	pub LevelEntity__BackingField: *const GameEntity,	// 0x80
	pub _PauseEntityTimeSlowIndexDic: *const NativeArray<NativeObject>,	// 0x88
	pub _AllTeamEntity: *const NativeArray<GameEntity>,	// 0x90
	pub DataViewUISelectFadeInEntity__BackingField: *const GameEntity,	// 0x98
	pub DataViewUISelectFadeInFollowEntities__BackingField: *const c_void,	// 0xa0
	pub PerformanceGORoot__BackingField: *const c_void,	// 0xa8
	pub DataViewUISelectFadeOutSummonerEntity__BackingField: *const GameEntity,	// 0xb0
	pub PlayerGORoot__BackingField: *const c_void,	// 0xb8
	pub DataViewUILeaveSummonerOfUncreatedServant__BackingField: *const GameEntity,	// 0xc0
	pub _UseUniqueSnapshot: bool	// 0xc8
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SkillData {
	pub native_object: NativeObject,
	pub OverrideAnimState: *const NativeString,	// 0x10
	pub DefaultTargetInfo: *const c_void,	// 0x18
	pub RowData: *const c_void,	// 0x20
	pub AllChildSkillDatas: *const NativeArray<SkillData>,	// 0x28
	pub Config: *const c_void,	// 0x30
	pub CustomReadyConfigConditions: *const NativeArray<NativeObject>,	// 0x38
	pub PreshowConditions: *const NativeArray<NativeObject>,	// 0x40
	pub _Slot: *const c_void,	// 0x48
	pub OverrideCameraConfigAdded: *const c_void,	// 0x50
	pub InsertCondTask: *const c_void,	// 0x58
	pub SkillCom: *const SkillCharacterComponent,	// 0x60
	pub _SkillProperties: *const NativeArray<NativeObject>,	// 0x68
	pub VisibleCondTask: *const c_void,	// 0x70
	pub ParentSkillData: *const SkillData,	// 0x78
	pub OverrideTargetInfo: *const c_void,	// 0x80
	pub OverrideCameraConfig: *const c_void,	// 0x88
	pub UsableCondTask: *const c_void,	// 0x90
	pub SkillTriggerKey: *const NativeString,	// 0x98
	pub CurrentCoolDown: i32,	// 0xa0
	pub LeftCastTimes: i32,	// 0xa4
	pub MaxCastTimes: i32,	// 0xa8
	pub SkillConfigID: u32,	// 0xac
	pub AttackDamageTypePreshowAttach: i32,	// 0xb0
	pub DefaultCoolDown: i32,	// 0xb4
	pub CommonActiveSkillID: u32,	// 0xb8
	pub ChildIndex: i32,	// 0xbc
	pub SkillIndex: i32	// 0xc0
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EntityType {
	None = 0,
	Avatar = 1,
	Monster = 2,
	LocalPlayer = 3,
	NPC = 4,
	NPCMonster = 5,
	StoryCharacter = 6,
	Prop = 7,
	Mission = 8,
	LevelEntity = 9,
	Neutral = 10,
	AtmoNpc = 11,
	BattleEvent = 12,
	TutorialEntity = 13,
	Team = 14,
	Partner = 15,
	LevelGraph = 16,
	Snapshot = 17,
	TeamFormation = 18,
	Model = 19,
	UICamera = 20,
	District = 21,
	GlobalShield = 22,
	CustomData = 23,
	Simple = 24,
	PuzzleGameObjectProp = 25,
	PerformanceLevelGraph = 26,
	Group = 27,
	ChessCharacter = 28,
	ChessTerrain = 29,
	SummonUnit = 30,
	LittleGameInstance = 31,
	Servant = 32,
	PreviewShow = 33,
	LittleGameContainer = 34,
	LittleGameViewProxy = 35,
	GridFightBackend = 36,
	DummyEntity = 37
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FixPoint {
	pub m_rawValue: i64	// 0x0
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AttackType {
	Unknown = 0,
	Normal = 1,
	BPSkill = 2,
	Ultra = 3,
	QTE = 4,
	DOT = 5,
	Pursued = 6,
	Maze = 7,
	MazeNormal = 8,
	Insert = 9,
	ElementDamage = 10,
	Level = 11,
	Servant = 12,
	TrueDamage = 13
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BattleLineupData {
	pub native_object: NativeObject,
	pub LightTeam: *const NativeArray<LineUpCharacter>,	// 0x10
	pub SpecialAvatarLevelAreaConfigs: *const c_void,	// 0x18
	pub ExtraTeam: *const NativeArray<LineUpCharacter>,	// 0x20
	pub Context: *const c_void,	// 0x28
	pub TeamBuffIDList: *const NativeArray<u32>,	// 0x30
	pub DeferCreateTrialPlayerDic: *const c_void,	// 0x38
	pub _LevelPath: *const NativeString,	// 0x40
	pub AdditionalTemplateVariables: *const c_void,	// 0x48
	pub MazeBuffAdded: *const NativeArray<NativeObject>,	// 0x50
	pub _TemplateVariables: *const c_void,	// 0x58
	pub BattleExtraPropertyAdditionDict__BackingField: *const c_void,	// 0x60
	pub WorldLevel: u32	// 0x68
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TurnBasedGameMode {
	pub native_object: NativeObject,
	pub _TurnStateFSM: *const c_void,	// 0x10
	pub _ActionEntityListSnapshot: *const NativeArray<GameEntity>,	// 0x18
	pub _UnselectableEntities: *const NativeArray<GameEntity>,	// 0x20
	pub _LimboEntitiesWaitAbilityFinish: *const NativeArray<NativeObject>,	// 0x28
	pub _LinkTeammateList: *const NativeArray<GameEntity>,	// 0x30
	pub CurrentTurnOwnerEntity__BackingField: *const GameEntity,	// 0x38
	pub _ModifierPerformCamerContext: *const c_void,	// 0x40
	pub TimeGameStart: *const c_void,	// 0x48
	pub _CurrentActionDelayModifyGroup: *const NativeArray<GameEntity>,	// 0x50
	pub _PhaseModifierList: *const NativeArray<NativeObject>,	// 0x58
	pub _ActionDelayOrderTrigger: *const c_void,	// 0x60
	pub _AvatarChangeParam: *const c_void,	// 0x68
	pub LastSummonMonsterList: *const NativeArray<GameEntity>,	// 0x70
	pub _AttackingEntityList: *const c_void,	// 0x78
	pub _ActionEntityList: *const NativeArray<GameEntity>,	// 0x80
	pub _CurrentSkillCharacter: *const SkillCharacterComponent,	// 0x88
	pub _ImmediateActionEntities: *const c_void,	// 0x90
	pub _AidDetail: *const c_void,	// 0x98
	pub _EntityCustomUnselectableDatas: *const NativeArray<NativeObject>,	// 0xa0
	pub _CommonSkillPoolNames: *const c_void,	// 0xa8
	pub SkillUsageLog__BackingField: *const c_void,	// 0xb0
	pub LastTurnSnapshot: *const c_void,	// 0xb8
	pub _LevelLockedFeatureSet: *const c_void,	// 0xc0
	pub _InsertAbilityList: *const NativeArray<MMNDIEBMDNL>,	// 0xc8
	pub BattleAIPublicKnowledge__BackingField: *const c_void,	// 0xd0
	pub StageBattleEventMgr__BackingField: *const c_void,	// 0xd8
	pub CurrentWaveMainMonsterIDPool__BackingField: *const NativeArray<u32>,	// 0xe0
	pub LastKillCaster__BackingField: *const GameEntity,	// 0xe8
	pub _RogueInBattleData: *const c_void,	// 0xf0
	pub OwnerBattleInstanceRef__BackingField: *const c_void,	// 0xf8
	pub _LimboEntities: *const NativeArray<NativeObject>,	// 0x100
	pub ActionBarMgr__BackingField: *const c_void,	// 0x108
	pub _SkillAddBuffPerformList: *const NativeArray<NativeObject>,	// 0x110
	pub PerformDelayExecuteList: *const NativeArray<NativeObject>,	// 0x118
	pub _EntityModifierPerforms: *const c_void,	// 0x120
	pub _InsertUltraSkillParamsQueue: *const NativeArray<NativeObject>,	// 0x128
	pub _LastBreakMonster: *const GameEntity,	// 0x130
	pub _ReplayData: *const c_void,	// 0x138
	pub DamageQueue__BackingField: *const c_void,	// 0x140
	pub BattleChangeAvatarManager__BackingField: *const c_void,	// 0x148
	pub PrepareAbility__BackingField: *const c_void,	// 0x150
	pub _EvolveBuildGearMgr: *const c_void,	// 0x158
	pub _CurrentTurnActionEntity: *const GameEntity,	// 0x160
	pub _SomatoModifierPerforms: *const NativeArray<NativeObject>,	// 0x168
	pub _VersusBarMgr: *const c_void,	// 0x170
	pub _performParam: *const c_void,	// 0x178
	pub AssistantAvatarEntity__BackingField: *const GameEntity,	// 0x180
	pub _RelationGroupMgr: *const c_void,	// 0x188
	pub _EventProcessor: *const c_void,	// 0x190
	pub _allowQuitStates: *const NativeArray<NativeObject>,	// 0x198
	pub _SwordTrainingMgr: *const c_void,	// 0x1a0
	pub _AllOffTeamCharacters: *const NativeArray<GameEntity>,	// 0x1a8
	pub _AllTeamCharacters: *const NativeArray<GameEntity>,	// 0x1b0
	pub LastKillSkill__BackingField: *const c_void,	// 0x1b8
	pub GridFightMananger__BackingField: *const c_void,	// 0x1c0
	pub TurnActionDelayCostChangeSource__BackingField: *const GameEntity,	// 0x1c8
	pub BattleCounter: *const c_void,	// 0x1d0
	pub MonsterWaveTextInfo: *const c_void,	// 0x1d8
	pub _LimboEntitiesSkipSettlement: *const NativeArray<NativeObject>,	// 0x1e0
	pub LastKillTargetList__BackingField: *const NativeArray<GameEntity>,	// 0x1e8
	pub _CurModifierPerformSeq: *const c_void,	// 0x1f0
	pub BattleEventInitedData__BackingField: *const c_void,	// 0x1f8
	pub CurrentMVPEntity__BackingField: *const GameEntity,	// 0x200
	pub _ActionDelayChangeStamp: [u8; 0x18],	// 0x208
	pub _LimboRevivableEntities: *const NativeArray<NativeObject>,	// 0x220
	pub _WaitingAbilityList: *const NativeArray<NativeObject>,	// 0x228
	pub _CachedDynamicSkillTargetSelection: *const GameEntity,	// 0x230
	pub _ActionDelayLinkMgr: *const c_void,	// 0x238
	pub _MainMonster: *const GameEntity,	// 0x240
	pub ThisTurnAnimEvents: *const c_void,	// 0x248
	pub _CurrentTurnTargetEntity: *const GameEntity,	// 0x250
	pub LastZombie__BackingField: *const GameEntity,	// 0x258
	pub _OverrieWaveMonsterPerformDatas: *const NativeArray<NativeObject>,	// 0x260
	pub CurrentWaveIndexInStage__BackingField: u32,	// 0x268
	pub ClearUltraSkillEffect: bool,	// 0x26c
	pub _IsModifierPerformCameraSet: bool,	// 0x26d
	pub AddOpCountOnInsertUltraWaitOrder: bool,	// 0x26e
	pub LocalWinFlag__BackingField: [u8; 0x5],	// 0x26f
	pub ChallengeTurnLimitType__BackingField: i32,	// 0x274
	pub BattleResultState__BackingField: *const c_void,	// 0x278
	pub _PrevTickModeState: i32,	// 0x280
	pub CurrentModeState__BackingField: i32,	// 0x284
	pub _ChallengeTurnAcc: u32,	// 0x288
	pub CurrentWaveStageID__BackingField: u32,	// 0x28c
	pub _SkillExecutionEventState: i32,	// 0x290
	pub UseSkillOneMoreDefaultSkill: i32,	// 0x294
	pub _TurnCounter: u32,	// 0x298
	pub _IsReplayBeingSaved: bool,	// 0x29c
	pub ApplyUIOperateOnDisableActionFlagChange: bool,	// 0x29d
	pub _IsUseLinkSkill: bool,	// 0x29e
	pub BattleResultAsWin: bool,	// 0x29f
	pub TurnActionDelayCostRatio__BackingField: FixPoint,	// 0x2a0
	pub _DeathVersion: u32,	// 0x2a8
	pub ClearUltraSkillQueue__BackingField: bool,	// 0x2ac
	pub AutoInsertUltraSkill: bool,	// 0x2ad
	pub SkipDeathHandle__BackingField: bool,	// 0x2ae
	pub PendingMonsterToWave__BackingField: bool,	// 0x2af
	pub CertainlyWinInAdvance__BackingField: bool,	// 0x2b0
	pub TurnOwnerActionPhaseEnd__BackingField: bool,	// 0x2b1
	pub IsLastKillTriggered: bool,	// 0x2b2
	pub WinFlag: bool,	// 0x2b3
	pub ThisTurnAnimEventCount: i32,	// 0x2b4
	pub _ModifierPerformTimeScale: f32,	// 0x2b8
	pub _LightTeamTurnCount: u32,	// 0x2bc
	pub IsUseSkillOneMore: bool,	// 0x2c0
	pub SkipTurnOwnerActionFlag__BackingField: bool,	// 0x2c1
	pub _AutoBattle: bool,	// 0x2c2
	pub PrepareAbilityFinish__BackingField: bool,	// 0x2c3
	pub _OperationCounter: u32,	// 0x2c4
	pub SkipCameraDitherByLastKill: bool,	// 0x2c8
	pub LastKillFinish__BackingField: bool,	// 0x2c9
	pub MuteLastKillTriggered: bool,	// 0x2ca
	pub _OverrideAILocked: bool,	// 0x2cb
	pub PropagateBeingAttackTeam__BackingField: TeamType,	// 0x2cc
	pub _WaveMonsterCurrentCount: i32,	// 0x2d0
	pub _HitPerformMinTimer: f32,	// 0x2d4
	pub _NextModifierIndex: i32,	// 0x2d8
	pub _CurrentTurnTeam: TeamType,	// 0x2dc
	pub _DarkTeamTurnCount: u32,	// 0x2e0
	pub StanceCountDownSPChangeValue__BackingField: f32,	// 0x2e4
	pub _DamageCounter: u32,	// 0x2e8
	pub _ModifierPerformTimerTotal: f32,	// 0x2ec
	pub PauseState__BackingField: i32,	// 0x2f0
	pub IsActionOrder1UsedTBSkill__BackingField: bool,	// 0x2f4
	pub _LastReplayAutoBattle: bool,	// 0x2f5
	pub _CachedDynamicSkillInput: i32,	// 0x2f8
	pub _RecordOperationByLG: [u8; 0xc],	// 0x2fc
	pub ElapsedActionDelay__BackingField: FixPoint,	// 0x308
	pub ShowCutinUIState__BackingField: i32,	// 0x310
	pub _NextAbilityIndex: i32,	// 0x314
	pub _HoldFrameForCapture: u32,	// 0x318
	pub _IsCreatingNewWave: bool,	// 0x31c
	pub TurnOwnerPrepareAbilityUsed__BackingField: bool,	// 0x31d
	pub _ActionEntityListInited: bool,	// 0x31e
	pub RealTimeCounter__BackingField: f32,	// 0x320
	pub ChallengeTurnLimit__BackingField: u32,	// 0x324
	pub _ModifierEndingPerformRemainedTime: f32,	// 0x328
	pub BattleFinishReason: i32,	// 0x32c
	pub _ModifierPerformTimeTotal: f32,	// 0x330
	pub TurnEndKeep: bool,	// 0x334
	pub _GamePauseFlag: bool,	// 0x335
	pub IsManualRetryExitBattle: bool,	// 0x336
	pub CertainlyLoseInAdvance__BackingField: bool,	// 0x337
	pub CurrentInsertSkillSkipActionFlag: bool,	// 0x338
	pub _HoldFrameForCaptureFlag: bool,	// 0x339
	pub IsManualExitBattle: bool,	// 0x33a
	pub ApplyUIOperateOnSkillDisableChange: bool,	// 0x33b
	pub ForbidAI: bool,	// 0x33c
	pub _RequireMakeLimboEntitiesDie: bool,	// 0x33d
	pub _CurrentTurnActionEntitySkipActionFlag: bool,	// 0x33e
	pub IsTeamFormationExpansion__BackingField: bool,	// 0x33f
	pub WaveMonsterMaxCount__BackingField: i32	// 0x340
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GameEntity {
	pub native_object: NativeObject,
	pub _TickLodProxy: *const c_void,	// 0x10
	pub OnTeamChange: *const c_void,	// 0x18
	pub _UnstageReasonKey: *const NativeString,	// 0x20
	pub OnStageStateChange: *const c_void,	// 0x28
	pub _UnityGO: *const c_void,	// 0x30
	pub _DestroyWaitList: *const c_void,	// 0x38
	pub _OwnerWorldRef: *const c_void,	// 0x40
	pub TimeScaleStack: *const c_void,	// 0x48
	pub HoyoTagContainer: *const c_void,	// 0x50
	pub NameForGameCore__BackingField: *const NativeString,	// 0x58
	pub _ComponentArray: *const NativeArray<GameComponentBase>,	// 0x60
	pub TickLodTemplate: *const NativeString,	// 0x68
	pub WorldTimeScaleAdpator: *const c_void,	// 0x70
	pub DisposeCallback: *const c_void,	// 0x78
	pub _LateUpdateComponentList: *const c_void,	// 0x80
	pub _TickComponentList: *const c_void,	// 0x88
	pub _ComponentList: *const c_void,	// 0x90
	pub _ComponentArrayRef: *const c_void,	// 0x98
	pub _CurTickListRef: [u8; 0x10],	// 0xa0
	pub TagComponentContainer: *const c_void,	// 0xb0
	pub Name__BackingField: *const NativeString,	// 0xb8
	pub _EntityType: EntityType,	// 0xc0
	pub _TickDelayFrameCount: u32,	// 0xc4
	pub LastTickBucket__BackingField: i32,	// 0xc8
	pub _IsRegisterEnviroChara: bool,	// 0xcc
	pub IsStoryMode__BackingField: bool,	// 0xcd
	pub HasDisposed: bool,	// 0xce
	pub IsHero__BackingField: bool,	// 0xcf
	pub LastTickTime__BackingField: f32,	// 0xd0
	pub _GroupEntityID: u32,	// 0xd4
	pub Disposing: bool,	// 0xd8
	pub IsLoaded__BackingField: bool,	// 0xd9
	pub IsFakeAvatar__BackingField: bool,	// 0xda
	pub _Tickable: bool,	// 0xdb
	pub TickLodBoundSize__BackingField: f32,	// 0xdc
	pub RuntimeID__BackingField: u32,	// 0xe0
	pub KillImmediately: bool,	// 0xe4
	pub _IsOnStage: bool,	// 0xe5
	pub Visible__BackingField: bool,	// 0xe6
	pub _ShouldLateUpdate: bool,	// 0xe7
	pub CampID__BackingField: i32,	// 0xe8
	pub _ServerEntityID: u32,	// 0xec
	pub LastTickFrame__BackingField: u64,	// 0xf0
	pub _Team: TeamType,	// 0xf8
	pub ForceIgnoreTickLodBistSet: u32,	// 0xfc
	pub _ForceTickLodLowestReason: *const c_void,	// 0x100
	pub _AliveState: i32,	// 0x108
	pub ObjectFeature__BackingField: i32,	// 0x10c
	pub _GroupID: u32	// 0x110
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BattleEventDataComponent {
	pub _parent_object:  CharacterDataComponent,	// 0x0
	pub _BattleEventRowData: *const c_void,	// 0x98
	pub CreateParams__BackingField: *const c_void,	// 0xa0
	pub BattleEventConfig__BackingField: *const c_void,	// 0xa8
	pub _EnergyBarState: *const c_void,	// 0xb0
	pub _TBAbilityRef: *const TurnBasedAbilityComponent,	// 0xb8
	pub SourceCaster__BackingField: *const GameEntity,	// 0xc0
	pub BattleEventTotalDamageType: TeamType,	// 0xc8
	pub WarningChallengeTurnLeft: u32	// 0xcc
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AbilityConfig {
	pub _parent_object: NativeObject,	// 0x0
	pub Name: *const NativeString,	// 0x10
	pub TargetInfo: *const c_void,	// 0x18
	pub OnAdd: *const NativeArray<NativeObject>,	// 0x20
	pub OnRemove: *const NativeArray<NativeObject>,	// 0x28
	pub OnStart: *const NativeArray<NativeObject>,	// 0x30
	pub DynamicValues: *const c_void,	// 0x38
	pub TaskListTemplate: *const NativeArray<NativeObject>,	// 0x40
	pub _TaskListTemplatesMap: *const c_void	// 0x48
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ServantSkillRowData {
	pub native_object: NativeObject,
	pub _Row: *const c_void,	// 0x10
	pub _DefaultOverrideData: [u8; 0xe8],	// 0x18
	pub _Config: *const c_void,	// 0x100
	pub _OverrideData: [u8; 0xe0]	// 0x108
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TeamType {
	TeamUnknow = 0,
	TeamLight = 1,
	TeamDark = 2,
	TeamNeutral = 3,
	TeamNPC = 4,
	Count = 5
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CharacterConfig {
	pub _parent_object: NativeObject,	// 0x0
	pub SomatoType: i32,	// 0x10
	pub CharacterBodySize: i32,	// 0x14
	pub CharacterHUDOffset: [u8; 0xc],	// 0x18
	pub BuffPanelOffset: [u8; 0xc],	// 0x24
	pub HitBoxOffset: [u8; 0xc],	// 0x30
	pub TargetSelectGroup: i32,	// 0x3c
	pub CameraConfigList: *const NativeArray<NativeObject>,	// 0x40
	pub HitBoxType: i32,	// 0x48
	pub HitBoxWidth: f32,	// 0x4c
	pub HitBoxLength: f32,	// 0x50
	pub HitBoxHeight: f32,	// 0x54
	pub HitBoxAttachPoint: *const NativeString,	// 0x58
	pub Resilience: *const c_void,	// 0x60
	pub Location: *const c_void,	// 0x68
	pub VisualRadius: f32,	// 0x70
	pub LookAtIKEnableRadius: f32,	// 0x74
	pub AutoFlipModel: bool,	// 0x78
	pub SaveModelWhenDead: bool,	// 0x79
	pub DeadPerform: bool,	// 0x7a
	pub PreloadUltraSkill: bool,	// 0x7b
	pub IsSpecialVisualCharacter: i32,	// 0x7c
	pub HideInTimeline: bool,	// 0x80
	pub AnimEventConfigList: *const NativeArray<NativeString>,	// 0x88
	pub SkillList: *const NativeArray<NativeObject>,	// 0x90
	pub AbilityList: *const NativeArray<NativeString>,	// 0x98
	pub SkillAbilityList: *const NativeArray<NativeObject>,	// 0xa0
	pub DynamicValues: *const c_void,	// 0xa8
	pub CustomValues: *const c_void,	// 0xb0
	pub WeaponType: i32,	// 0xb8
	pub ArmorType: i32,	// 0xbc
	pub SkillReadyTransits: *const NativeArray<NativeObject>,	// 0xc0
	pub PhaseAnimConfig: *const c_void,	// 0xc8
	pub AnimZoneConfigPath: *const NativeString,	// 0xd0
	pub InitAnimStateName: *const NativeString,	// 0xd8
	pub WhitelistSkillStateForInterrupt: *const NativeArray<NativeString>,	// 0xe0
	pub ModifierPerformTimeFactor: f32,	// 0xe8
	pub AsAidAttackTask: *const c_void,	// 0xf0
	pub AsAidDefenderTask: *const c_void,	// 0xf8
	pub AsAidProtectorTask: *const c_void,	// 0x100
	pub DisableAnimEventLayers: *const NativeArray<NativeString>,	// 0x108
	pub OnHitEditFootIKModeMap: *const c_void,	// 0x110
	pub RepeatOccurAnimWhenBeHitNormalizedTime: f32,	// 0x118
	pub CameraNamedDynamicOffset: *const NativeString,	// 0x120
	pub IgnoreDynamicOffsetBySelf: bool,	// 0x128
	pub OverrideHeightForCameraOffset: f32,	// 0x12c
	pub MonsterIgnoreGlobalDymanicOffset: bool,	// 0x130
	pub MaxMonsterPhase: u32,	// 0x134
	pub PhaseList: *const NativeArray<NativeObject>,	// 0x138
	pub OverrideWaveMonsterPerform: *const NativeString,	// 0x140
	pub OverrideColliderCameraByName: *const NativeString,	// 0x148
	pub EntityColliderConfig: *const c_void,	// 0x150
	pub EffectAdaptionList: *const NativeArray<NativeObject>,	// 0x158
	pub AttachPointEffectAdaptionList: *const NativeArray<NativeObject>,	// 0x160
	pub FieldEffectAdaptionList: *const NativeArray<NativeObject>,	// 0x168
	pub EffectAttachPointRedirect: *const c_void,	// 0x170
	pub MonsterConfig: *const c_void,	// 0x178
	pub ResidentEffectKey: *const NativeString,	// 0x180
	pub ResidentPossessionKey: *const NativeString,	// 0x188
	pub EmotionCharacterID: *const NativeString,	// 0x190
	pub GraphEmotionAsset: *const NativeString,	// 0x198
	pub AITagList: *const c_void,	// 0x1a0
	pub GlobalAIFactorGroups: *const c_void,	// 0x1a8
	pub ReplaceEmoConfig: *const c_void,	// 0x1b0
	pub WillUnstage: bool,	// 0x1b8
	pub ViewModeSortPriority: u32,	// 0x1bc
	pub ReplaceAnimtorControllerPath: *const NativeString,	// 0x1c0
	pub AlwaysCutOnSkillTargetTeamChange: bool	// 0x1c8
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AvatarSkillRowData {
	pub native_object: NativeObject,
	pub _Config: *const c_void,	// 0x10
	pub _OverrideData: [u8; 0xe8],	// 0x18
	pub _Row: *const c_void,	// 0x100
	pub _DefaultOverrideData: [u8; 0xe0]	// 0x108
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LineUpCharacter {
	pub native_object: NativeObject,
	pub BattleRelicItemModule: *const c_void,	// 0x10
	pub SpiritPassiveList: *const NativeArray<u32>,	// 0x18
	pub BattleEquipmentList: *const NativeArray<NativeObject>,	// 0x20
	pub SkillTreePointList: *const NativeArray<NativeObject>,	// 0x28
	pub ChangedSkillTreePointList: *const NativeArray<NativeObject>,	// 0x30
	pub BattleGridAvatarData: *const c_void,	// 0x38
	pub CharacterHPRatio: FixPoint,	// 0x40
	pub CharacterSP_Denominator: FixPoint,	// 0x48
	pub SpiritLineupType: i32,	// 0x50
	pub CharacterID: u32,	// 0x54
	pub AssistUid: u32,	// 0x58
	pub CharacterRank: u32,	// 0x5c
	pub CharacterAvatarType: i32,	// 0x60
	pub WorldLevel: u32,	// 0x64
	pub SpecialAvatarID: u32,	// 0x68
	pub CharacterRowIndex: u32,	// 0x6c
	pub Index: u32,	// 0x70
	pub TotalPower: u32,	// 0x74
	pub CharacterSP_Numerator: FixPoint,	// 0x78
	pub CharacterLevel: u32,	// 0x80
	pub CharacterPromotion: u32	// 0x84
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SkillCharacterComponent {
	pub _parent_object:  GameComponentBase,	// 0x0
	pub _CharacterDataRef: *const CharacterDataComponent,	// 0x18
	pub CurrentAimAtTargetList: *const NativeArray<GameEntity>,	// 0x20
	pub CurrentAimAtMainTargetList: *const NativeArray<GameEntity>,	// 0x28
	pub _SkillTypeDisableSlots: *const c_void,	// 0x30
	pub _SkillSlots: *const NativeArray<NativeObject>,	// 0x38
	pub SkillActualAttacker__BackingField: *const GameEntity,	// 0x40
	pub CurrentSkillTargetCharacterId: *const c_void,	// 0x48
	pub _SkillTargetRedirectEntries: *const NativeArray<NativeObject>,	// 0x50
	pub SkillPointEntity__BackingField: *const GameEntity,	// 0x58
	pub CurrentSkillTargetDamageHP: *const c_void,	// 0x60
	pub CurrentSkillTargetList__BackingField: *const NativeArray<GameEntity>,	// 0x68
	pub _SkillDataList: *const NativeArray<SkillData>,	// 0x70
	pub _JsonConfigRef: *const CharacterConfig,	// 0x78
	pub _SkillTypeDisableCountArr: *const NativeArray<i32>,	// 0x80
	pub CurrentAimAtSubTargetList: *const NativeArray<GameEntity>,	// 0x88
	pub _recordAbilityInfo: [u8; 0x30],	// 0x90
	pub TaskContext__BackingField: *const c_void,	// 0xc0
	pub OnSkillSetup: *const NativeArray<NativeObject>,	// 0xc8
	pub CurrentSkillSubTargetList__BackingField: *const NativeArray<GameEntity>,	// 0xd0
	pub _TBAbilityRef: *const TurnBasedAbilityComponent,	// 0xd8
	pub AutoUseUltraParams: *const c_void,	// 0xe0
	pub _OpIndexInSkill: i32,	// 0xe8
	pub _CurrentSkillIndex: i32,	// 0xec
	pub CurrentSkillHasTriggerEffect: bool,	// 0xf0
	pub IsNoBpCost__BackingField: bool,	// 0xf1
	pub CurrentSkillBreakStance: bool,	// 0xf2
	pub _isPassive: bool,	// 0xf3
	pub CurrentSkillKilledCount: i32,	// 0xf4
	pub _RedirectTargetIDIncr: i32,	// 0xf8
	pub _RecordSkillExtraUseParam: i32,	// 0xfc
	pub _TargetPerformTimeCounter: f32,	// 0x100
	pub _actionSkillIndex: i32,	// 0x104
	pub _SelfSkillPerformState: i32,	// 0x108
	pub SelfWaitActiveSkillIndex: i32,	// 0x10c
	pub _CurrentSkillExtraUseParam: i32,	// 0x110
	pub _AutoStandbyOnCurSkillFinish: bool,	// 0x114
	pub _hasOpInSkill: bool,	// 0x115
	pub CharmAction: bool,	// 0x116
	pub CurrentSkillKillAllOrBoss: bool,	// 0x117
	pub _hasRecordSkill: bool	// 0x118
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GameComponentBase {
	pub native_object: NativeObject,
	pub _OwnerRef: *const GameEntity	// 0x10
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BattleEventSkillRowData {
	pub native_object: NativeObject,
	pub _Config: *const c_void,	// 0x10
	pub _OverrideData: [u8; 0xe8],	// 0x18
	pub _Row: *const c_void,	// 0x100
	pub _DefaultOverrideData: [u8; 0xe0]	// 0x108
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TurnBasedAbilityComponent {
	pub _parent_object:  GameComponentBase,	// 0x0
	pub LockActionDelayChange: *const c_void,	// 0x18
	pub _KillerEntity: *const GameEntity,	// 0x20
	pub _DmgChunk: *const c_void,	// 0x28
	pub OverflowStanceDamageAttacker__BackingField: *const GameEntity,	// 0x30
	pub _DamagedAllEntityIDListInAttack: *const c_void,	// 0x38
	pub AbilityComponentRef__BackingField: *const c_void,	// 0x40
	pub _ModifierEventSourceMuteCounter: *const c_void,	// 0x48
	pub DamageDefender: *const GameEntity,	// 0x50
	pub _LockHPList: *const NativeArray<NativeObject>,	// 0x58
	pub _ModifierDelayParamList: *const c_void,	// 0x60
	pub DamageSplitData: *const NativeArray<NativeObject>,	// 0x68
	pub _ExtraMaxLayerConfig: *const NativeArray<NativeObject>,	// 0x70
	pub _DotModifierEventProcessors: *const NativeArray<NativeObject>,	// 0x78
	pub _AbilityPropertiesInitSnapshot: *const NativeArray<FixPoint>,	// 0x80
	pub _AbilityProperties: *const NativeArray<NativeObject>,	// 0x88
	pub _DefaultStanceInfo: *const c_void,	// 0x90
	pub _SelfExtrAbilityList: *const NativeArray<NativeString>,	// 0x98
	pub LastStanceBreakEntity__BackingField: *const GameEntity,	// 0xa0
	pub AdditionalAbilityParamList: *const NativeArray<NativeObject>,	// 0xa8
	pub ModifierOverrideMapping: *const c_void,	// 0xb0
	pub DisableActionStateByTask__BackingField: *const c_void,	// 0xb8
	pub _StatusProbabilityDict: *const c_void,	// 0xc0
	pub _BuffLockStepSources: *const NativeArray<NativeObject>,	// 0xc8
	pub ResistModifierBehaviorFlags__BackingField: *const NativeArray<NativeObject>,	// 0xd0
	pub RegardAsAttackTypeMap: *const NativeArray<NativeObject>,	// 0xd8
	pub AddModifierBindValueMapping: *const c_void,	// 0xe0
	pub _TransformRef: *const c_void,	// 0xe8
	pub _StancePreshowConfigs: *const NativeArray<NativeObject>,	// 0xf0
	pub _LockShieldCounter: *const c_void,	// 0xf8
	pub ProjectileTargetAttachPoint: *const NativeString,	// 0x100
	pub OnAbilityPropertyChanged: *const NativeArray<NativeObject>,	// 0x108
	pub KillerSkill__BackingField: *const c_void,	// 0x110
	pub _DebuffLockStepSources: *const NativeArray<NativeObject>,	// 0x118
	pub Weakness: *const c_void,	// 0x120
	pub _ModifierRecordList: *const c_void,	// 0x128
	pub _DelayModifyActionDelayQueue: *const c_void,	// 0x130
	pub _DepartedParams: *const NativeArray<NativeObject>,	// 0x138
	pub _RedStanceInfo: *const c_void,	// 0x140
	pub _EnergyPointEntries: *const NativeArray<NativeObject>,	// 0x148
	pub _SyncPropertySource: *const TurnBasedAbilityComponent,	// 0x150
	pub CharmDamageTarget: *const GameEntity,	// 0x158
	pub CustomDataRef__BackingField: *const c_void,	// 0x160
	pub _JsonConfigRef: *const CharacterConfig,	// 0x168
	pub _DamageStoreList: *const NativeArray<NativeObject>,	// 0x170
	pub _CharacterDataRef: *const CharacterDataComponent,	// 0x178
	pub _RedStanceInfoList: *const NativeArray<NativeObject>,	// 0x180
	pub CharmDamageAttackProperty: *const c_void,	// 0x188
	pub _DamageAttacker: *const GameEntity,	// 0x190
	pub _EnableNegativeHPSourceList: *const NativeArray<NativeObject>,	// 0x198
	pub _AbilityToSkillMapping: *const c_void,	// 0x1a0
	pub _ExtraStanceInfo: *const c_void,	// 0x1a8
	pub _SyncPropertyMask: *const c_void,	// 0x1b0
	pub _DamagedEntityListInAttack: *const NativeArray<GameEntity>,	// 0x1b8
	pub CharmSkillName: *const NativeString,	// 0x1c0
	pub _ModifierEventProcessors: *const NativeArray<NativeObject>,	// 0x1c8
	pub _StatusChanceResistanceDict: *const c_void,	// 0x1d0
	pub _OnHitEffectMultipleOverride: *const NativeArray<NativeObject>,	// 0x1d8
	pub RegardAsSkillTypeMap: *const NativeArray<NativeObject>,	// 0x1e0
	pub _OnHitEffectOverride: *const NativeArray<NativeObject>,	// 0x1e8
	pub ForbidVisualFlagValue__BackingField: i32,	// 0x1f0
	pub BlockModifySp__BackingField: bool,	// 0x1f4
	pub ForceKillFlag__BackingField: bool,	// 0x1f5
	pub CharmDisableSPAdd: bool,	// 0x1f6
	pub _IsBehaviorFlagVisualDirty: bool,	// 0x1f7
	pub LastBreakStanceDamageType__BackingField: i32,	// 0x1f8
	pub CharmDamageCount: i32,	// 0x1fc
	pub VisualFlagValue__BackingField: i32,	// 0x200
	pub _ResetStanceVersion: u32,	// 0x204
	pub StanceType: i32,	// 0x208
	pub IsTriggeringStanceCountDown__BackingField: bool,	// 0x20c
	pub PropertyChangeFlag__BackingField: bool,	// 0x20d
	pub _HighestPriorityOnHitEffect: i32,	// 0x210
	pub SpeedVisualFlagValue__BackingField: i32,	// 0x214
	pub OverflowStanceDamage__BackingField: FixPoint,	// 0x218
	pub IsTriggeredBlockDamage: bool,	// 0x220
	pub ActionDelayChanged__BackingField: [u8; 0x2],	// 0x221
	pub IsSharedDamageDataTarget: bool,	// 0x223
	pub HasRevived: bool,	// 0x224
	pub UseSpecialSP__BackingField: bool,	// 0x225
	pub _IsProcessingModifierDelayParam: bool,	// 0x226
	pub _BreakExtendEventUnsettled: bool,	// 0x227
	pub MuteAllTriggerDeath__BackingField: bool,	// 0x228
	pub TotalHitNum: FixPoint,	// 0x230
	pub ProjectileHitCount: i32,	// 0x238
	pub BattleTag__BackingField: i32,	// 0x23c
	pub TotalDamageCurrentAttack: FixPoint,	// 0x240
	pub InheritSPRatio: FixPoint,	// 0x248
	pub CurrentAttackType__BackingField: AttackType,	// 0x250
	pub LastStanceDamageType__BackingField: i32,	// 0x254
	pub ActionDelayDistance__BackingField: FixPoint,	// 0x258
	pub _DebuffLockStep: i32,	// 0x260
	pub _DeathVersion: u32,	// 0x264
	pub MuteTriggerDeath__BackingField: bool,	// 0x268
	pub bIsInCharmAction: bool,	// 0x269
	pub IsInAttack: bool,	// 0x26a
	pub TriggerBreakExtendLogic: bool,	// 0x26b
	pub DeathSource__BackingField: i32,	// 0x26c
	pub InsertAbilityCount: i32,	// 0x270
	pub PropertyEnumBoundary__BackingField: i32,	// 0x274
	pub _ModifierDelayAddCount: i32,	// 0x278
	pub CharmDisableBPAdd: bool,	// 0x27c
	pub LockSelfActionDelay: bool,	// 0x27d
	pub IsSnapshot__BackingField: bool,	// 0x27e
	pub _CurrentAttackPhase: i32,	// 0x280
	pub _ModifierUIOperationIncr: i32,	// 0x284
	pub _BuffLockStep: i32,	// 0x288
	pub StanceState__BackingField: i32	// 0x28c
}
	}
}
