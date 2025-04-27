use std::ffi::c_void;
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NOPBAAAGGLA {
	pub native_object: NativeObject,
	pub FKKDFMPMJHG: *const NativeArray<NativeObject>,	// 0x10
	pub MDEHKOOKJCK: *const NativeArray<NativeObject>,	// 0x18
	pub JKCOIOLCMEP: *const TurnBasedAbilityComponent,	// 0x20
	pub PBHCGDFPEED: *const c_void,	// 0x28
	pub HKFGOHGKOGK: *const c_void,	// 0x30
	pub BEAJGANIDLJ: *const c_void,	// 0x38
	pub KNDJNKNHFFG: *const TurnBasedAbilityComponent,	// 0x40
	pub AAHMMHBHMFN: [u8; 0x90],	// 0x48
	pub JODAJBNCCNP: *const NativeArray<NativeObject>,	// 0xd8
	pub LGGEDDMACDF: *const NativeString,	// 0xe0
	pub KODEDHBLGGH: FixPoint,	// 0xe8
	pub PGOHAIPOCNK: FixPoint,	// 0xf0
	pub PDCMJAMPJNL: FixPoint,	// 0xf8
	pub EBDJHPNOALL: FixPoint,	// 0x100
	pub MAKENPDPHDN: FixPoint,	// 0x108
	pub GIHPOCDLJOA: FixPoint,	// 0x110
	pub NEPGNKOMAAA: FixPoint,	// 0x118
	pub MJMDGNPPILN: FixPoint,	// 0x120
	pub MHEBPGAHFCB: FixPoint,	// 0x128
	pub KPELFJICFDH: FixPoint,	// 0x130
	pub GNMAKKBFOCH: FixPoint,	// 0x138
	pub CMNBOEIDAOD: FixPoint,	// 0x140
	pub GFFCEBJGABG: bool,	// 0x148
	pub IJJHMGEHMHB: bool,	// 0x149
	pub EJJMIFKCFHP: bool,	// 0x14a
	pub CAANBNCPACE: bool,	// 0x14b
	pub CFBOJBAJCEA: i32,	// 0x14c
	pub HCGBHCPHDKJ: FixPoint,	// 0x150
	pub POLANGDKOKH: FixPoint,	// 0x158
	pub DJCAFPFKOGP: FixPoint,	// 0x160
	pub GJNAGCJONAO: FixPoint,	// 0x168
	pub COIDNPMCCFG: FixPoint,	// 0x170
	pub HNJBAFCNNDD: FixPoint,	// 0x178
	pub ALOGNJIBIPG: FixPoint,	// 0x180
	pub OJGNIBKADHK: u32,	// 0x188
	pub JGHJIGOCPNP: i32,	// 0x18c
	pub ACDFHOGEMCC: [u8; 0x40],	// 0x190
	pub CGMHNNNOKAI: FixPoint,	// 0x1d0
	pub PJPKDAKBEJI: FixPoint,	// 0x1d8
	pub IAAJMHADJDG: FixPoint,	// 0x1e0
	pub FGIPOLJPICM: FixPoint,	// 0x1e8
	pub PAIGBKBOKDI: FixPoint,	// 0x1f0
	pub FMMBMJKNAHI: FixPoint,	// 0x1f8
	pub MKIMEBNOEGI: FixPoint,	// 0x200
	pub GBENLNNEIJM: FixPoint,	// 0x208
	pub DKOIGIHEBCD: FixPoint,	// 0x210
	pub PJNEJPNBNMP: FixPoint,	// 0x218
	pub OHBMMFAFMDP: FixPoint,	// 0x220
	pub KOEGLFLGADD: FixPoint,	// 0x228
	pub FNDCNMHMCIC: FixPoint,	// 0x230
	pub GMBACFCLEGD: FixPoint,	// 0x238
	pub DEOICHHPAIF: FixPoint,	// 0x240
	pub NHHNLMOBEGH: FixPoint,	// 0x248
	pub AHOCGHANMCE: FixPoint,	// 0x250
	pub HJAEPANAFLN: FixPoint,	// 0x258
	pub MNGPDEOEHPE: FixPoint,	// 0x260
	pub MKNDMBOCCBO: FixPoint,	// 0x268
	pub AHHEDGLMDMG: *const c_void,	// 0x270
	pub ELGMFJLGCPH: FixPoint,	// 0x278
	pub JEHMOKDJDDE: FixPoint,	// 0x280
	pub BJAEJMLMJCL: FixPoint,	// 0x288
	pub APDDLHNGGIM: AttackType,	// 0x290
	pub KMIKODLPNGL: i32,	// 0x294
	pub CAILJEGIDKL: FixPoint,	// 0x298
	pub EFAAJEAENFF: FixPoint,	// 0x2a0
	pub ILNAKPIOOAK: FixPoint,	// 0x2a8
	pub DJHDAOOEJOF: FixPoint,	// 0x2b0
	pub DPPDEDGCLJJ: FixPoint,	// 0x2b8
	pub GAALBDHLFOG: FixPoint,	// 0x2c0
	pub HMMMDOHLFEP: FixPoint,	// 0x2c8
	pub CCLFKIPGJOG: FixPoint,	// 0x2d0
	pub JFMADBFKBDK: FixPoint,	// 0x2d8
	pub FLMEBELNIKK: FixPoint,	// 0x2e0
	pub GLGFEKEMMJJ: FixPoint,	// 0x2e8
	pub BGBOFNMKDNJ: FixPoint,	// 0x2f0
	pub GHBPOPKEGLE: FixPoint,	// 0x2f8
	pub MGFECPHDPHB: FixPoint,	// 0x300
	pub EBDJIHNKAOC: FixPoint,	// 0x308
	pub JHOHCEFOJNB: FixPoint,	// 0x310
	pub MKMILJKLJON: [u8; 0x58],	// 0x318
	pub GCNOMMHFPOG: FixPoint,	// 0x370
	pub APDLLHIMMEM: FixPoint,	// 0x378
	pub BKIFAEKCIHN: FixPoint,	// 0x380
	pub ABIPIIBIIBE: FixPoint,	// 0x388
	pub BDGDFKGOLPJ: FixPoint,	// 0x390
	pub AHPFPMEGEKG: bool,	// 0x398
	pub MNAPDDFFHJF: bool,	// 0x399
	pub KDCHAHHPPGD: bool,	// 0x39a
	pub KBKGNDFAKGD: bool,	// 0x39b
	pub COKMLMJPKLH: u32,	// 0x39c
	pub GOHOJAIMDNM: FixPoint,	// 0x3a0
	pub CINNHMENLIJ: FixPoint,	// 0x3a8
	pub BDLFBDLDEND: [u8; 0x48],	// 0x3b0
	pub GCFCCDPIACO: FixPoint,	// 0x3f8
	pub EPJEDLOBFPG: FixPoint,	// 0x400
	pub MLKFKKACBCE: FixPoint,	// 0x408
	pub DGFBMAPFPNH: FixPoint,	// 0x410
	pub NCOHIAPKAED: FixPoint,	// 0x418
	pub HHEIPBOKCOH: [u8; 0x40],	// 0x420
	pub JCPEINMPKAM: FixPoint,	// 0x460
	pub PGGOANFBJON: FixPoint,	// 0x468
	pub LJGPDLDGCEO: FixPoint,	// 0x470
	pub JNFPCNAKNOP: FixPoint,	// 0x478
	pub NAGMKEABGEE: FixPoint,	// 0x480
	pub KDJBABPDHEG: FixPoint,	// 0x488
	pub ANHNDBECCJD: [u8; 0x40],	// 0x490
	pub ODBPMMGBKGA: FixPoint,	// 0x4d0
	pub PJLPGAGKIDE: FixPoint,	// 0x4d8
	pub DBNKBGKCMKH: FixPoint,	// 0x4e0
	pub KLMAGCLFBAO: FixPoint,	// 0x4e8
	pub PNGJIDMHIOE: FixPoint,	// 0x4f0
	pub EFFODBPOOCN: FixPoint,	// 0x4f8
	pub JFKEEOMKMLI: FixPoint,	// 0x500
	pub IICNDPJGCFA: i32,	// 0x508
	pub DPEJKHJPLAC: bool,	// 0x50c
	pub JICCOEHBPJJ: bool,	// 0x50d
	pub BBDANLEJCIA: bool,	// 0x50e
	pub HKNLHAMMIIM: bool,	// 0x50f
	pub FFFOLNDHIEH: [u8; 0x48],	// 0x510
	pub EKBHFCODKFO: bool,	// 0x558
	pub HEMFDDDJOGK: bool,	// 0x559
	pub FNBALMGFGDM: bool,	// 0x55a
	pub EGINKGPDNPK: bool,	// 0x55b
	pub GCGEEFLGCIG: i32,	// 0x55c
	pub JIINJMJGCOH: FixPoint,	// 0x560
	pub DCEBGGFOFAO: FixPoint,	// 0x568
	pub DINCHAHPEAC: FixPoint,	// 0x570
	pub BBNMJNPDOCP: FixPoint,	// 0x578
	pub BLFCEOMPDKK: FixPoint,	// 0x580
	pub GLPLDJKMOBE: FixPoint,	// 0x588
	pub OEPAPFDLMML: FixPoint,	// 0x590
	pub FOLCDHNIMMI: FixPoint,	// 0x598
	pub ENFFBMJBEDP: FixPoint,	// 0x5a0
	pub AMAJNHHAJIM: FixPoint,	// 0x5a8
	pub DBBDIMCJIKE: FixPoint,	// 0x5b0
	pub BEGDMOGLLGM: FixPoint,	// 0x5b8
	pub FFCGIMAMDPP: FixPoint,	// 0x5c0
	pub KOCOLHHLFLD: [u8; 0x40]	// 0x5c8
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MMNDIEBMDNL {
	pub native_object: NativeObject,
	pub MKMMNLODHDD: *const c_void,	// 0x10
	pub FIMNOPAAFEP: *const TurnBasedAbilityComponent,	// 0x18
	pub HECCDOHIAFD: *const SkillCharacterComponent,	// 0x20
	pub GNBEIGMFGIP: *const c_void,	// 0x28
	pub HMCDHMFHABF: OLHMAHMMBNN,	// 0x30
	pub DADCNHAIOMI: i32,	// 0x88
	pub OOIFIGDBNBO: i32,	// 0x8c
	pub NMJEMHAMIHD: i32	// 0x90
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
pub mod rpg {
	use std::ffi::c_void;
	use crate::kreide::types::*;
	pub mod client {
	use std::ffi::c_void;
	use crate::kreide::types::*;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextID {
	pub hash: i32,	// 0x0
	pub hash64: u64	// 0x8
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AvatarData {
	pub native_object: NativeObject,
	pub GrowUpBeforeData__BackingField: *const c_void,	// 0x10
	pub Row__BackingField: *const c_void,	// 0x18
	pub _AvatarName: *const NativeString,	// 0x20
	pub ServantData__BackingField: *const AvatarServantData,	// 0x28
	pub LevelUpedBeforeData__BackingField: *const c_void,	// 0x30
	pub CombatPowerData__BackingField: *const c_void,	// 0x38
	pub SpecialRow__BackingField: *const c_void,	// 0x40
	pub UltraSkillConfig__BackingField: *const c_void,	// 0x48
	pub _TrialEquipment: *const c_void,	// 0x50
	pub AvatarPropertyData__BackingField: *const c_void,	// 0x58
	pub _ExtraPropertyAddition: *const c_void,	// 0x60
	pub SkillTreeData: *const c_void,	// 0x68
	pub _AvatarRowData: *const c_void,	// 0x70
	pub PromotedBeforeData__BackingField: *const c_void,	// 0x78
	pub HasTakenPromotionRewardList__BackingField: *const NativeArray<u32>,	// 0x80
	pub _SkillDataMap: *const c_void,	// 0x88
	pub _SkinIDList: *const NativeArray<u32>,	// 0x90
	pub RelicsData__BackingField: *const c_void,	// 0x98
	pub CurrentExp__BackingField: u32,	// 0xa0
	pub AvatarType__BackingField: i32,	// 0xa4
	pub IsNew__BackingField: bool,	// 0xa8
	pub IsDisplayOnly__BackingField: bool,	// 0xa9
	pub IsMarked__BackingField: bool,	// 0xaa
	pub RealID__BackingField: u32,	// 0xac
	pub EquipmentUID__BackingField: u32,	// 0xb0
	pub Promotion__BackingField: u32,	// 0xb4
	pub _BaseID: u32,	// 0xb8
	pub _AdventurePlayerID: u32,	// 0xbc
	pub Level__BackingField: u32,	// 0xc0
	pub SpecialAvatarID__BackingField: u32,	// 0xc4
	pub Rank__BackingField: u32,	// 0xc8
	pub DressedSkinID__BackingField: u32,	// 0xcc
	pub FirstMetTimeStamp: u64	// 0xd0
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModuleManager {
	pub native_object: NativeObject,
	pub MultipleDropModule: *const c_void,	// 0x10
	pub FindChestModule: *const c_void,	// 0x18
	pub TutorialSupportModule: *const c_void,	// 0x20
	pub ShareModule: *const c_void,	// 0x28
	pub SwitchHandModule: *const c_void,	// 0x30
	pub RoleTrialModule: *const c_void,	// 0x38
	pub ActivityMusicRhythmModule: *const c_void,	// 0x40
	pub MusicAlbumModule: *const c_void,	// 0x48
	pub ItemComposeModule: *const c_void,	// 0x50
	pub ActivityFeverTimeModule: *const c_void,	// 0x58
	pub RogueTournModule: *const c_void,	// 0x60
	pub TitanAtlasModule: *const c_void,	// 0x68
	pub DifficultyAdjustModule: *const c_void,	// 0x70
	pub RechargeShopModule: *const c_void,	// 0x78
	pub ActivityQuestTimeLimitModule: *const c_void,	// 0x80
	pub SystemOpenModule: *const c_void,	// 0x88
	pub ToastQueueModule: *const c_void,	// 0x90
	pub WorldShop4ThModule: *const c_void,	// 0x98
	pub PhotoGraphModule: *const c_void,	// 0xa0
	pub MissionModule: *const c_void,	// 0xa8
	pub StarFightModule: *const c_void,	// 0xb0
	pub ActivityTrackPhotoModule: *const c_void,	// 0xb8
	pub MovieRacingModule: *const c_void,	// 0xc0
	pub MultiPathAvatarModule: *const c_void,	// 0xc8
	pub TravelBrochureModule: *const c_void,	// 0xd0
	pub OperationModule: *const c_void,	// 0xd8
	pub MaterialSubmissionModule: *const c_void,	// 0xe0
	pub ActivitySummonModule: *const c_void,	// 0xe8
	pub LoginModule: *const c_void,	// 0xf0
	pub TalkModule: *const c_void,	// 0xf8
	pub ServerPrefsModule: *const c_void,	// 0x100
	pub MuseumModule: *const c_void,	// 0x108
	pub FantasticStoryActivityModule: *const c_void,	// 0x110
	pub MonopolyModule: *const c_void,	// 0x118
	pub GridFightModule: *const c_void,	// 0x120
	pub EntityScoreModule: *const c_void,	// 0x128
	pub PlanetFesModule: *const c_void,	// 0x130
	pub GamePlayLockModule: *const c_void,	// 0x138
	pub RelicModule: *const c_void,	// 0x140
	pub RogueArcadeModule: *const c_void,	// 0x148
	pub FiveDimModule: *const c_void,	// 0x150
	pub AnniversaryAvatarDeliverModule: *const c_void,	// 0x158
	pub BattleModule: *const c_void,	// 0x160
	pub PersonalizeModule: *const c_void,	// 0x168
	pub AdventureModule: *const c_void,	// 0x170
	pub ShopModule: *const c_void,	// 0x178
	pub WolfBroShootingModule: *const c_void,	// 0x180
	pub FightFestModule: *const c_void,	// 0x188
	pub DialogueModule: *const c_void,	// 0x190
	pub MapConnectivityModule: *const c_void,	// 0x198
	pub HeliobusModule: *const c_void,	// 0x1a0
	pub BattleTipsModule: *const c_void,	// 0x1a8
	pub ChallengeModule: *const c_void,	// 0x1b0
	pub ExpeditionModule: *const c_void,	// 0x1b8
	pub PerformanceRecallModule: *const c_void,	// 0x1c0
	pub SilverWolfModule: *const c_void,	// 0x1c8
	pub BattleCollegeModule: *const c_void,	// 0x1d0
	pub AetherDivideModule: *const c_void,	// 0x1d8
	pub BigMapModule: *const c_void,	// 0x1e0
	pub CompanionMissionActivityModule: *const c_void,	// 0x1e8
	pub PlayerModule: *const c_void,	// 0x1f0
	pub MultiPlayerActivityModule: *const c_void,	// 0x1f8
	pub TrainPartyModule: *const c_void,	// 0x200
	pub EvolveBuildModule: *const c_void,	// 0x208
	pub MultiFloorConflictModule: *const c_void,	// 0x210
	pub RollShopModule: *const c_void,	// 0x218
	pub ChatModule: *const c_void,	// 0x220
	pub CumulativeConsumptionModule: *const c_void,	// 0x228
	pub HandbookModule: *const c_void,	// 0x230
	pub FeatureSwitchModule: *const c_void,	// 0x238
	pub ActivityModule: *const c_void,	// 0x240
	pub LobbyModule: *const c_void,	// 0x248
	pub MatchThreeModule: *const c_void,	// 0x250
	pub MapPropOverrideConditionModule: *const c_void,	// 0x258
	pub ActivitySwordTrainingModule: *const c_void,	// 0x260
	pub NovelModule: *const c_void,	// 0x268
	pub FormationMoveModule: *const c_void,	// 0x270
	pub DrinkMakerModule: *const c_void,	// 0x278
	pub RecommendModule: *const c_void,	// 0x280
	pub EarlyAccessModule: *const c_void,	// 0x288
	pub TeamModule: *const c_void,	// 0x290
	pub AntiAddictionModule: *const c_void,	// 0x298
	pub FarmModule: *const c_void,	// 0x2a0
	pub TransferModule: *const c_void,	// 0x2a8
	pub CatchGhostModule: *const c_void,	// 0x2b0
	pub MessageModule: *const c_void,	// 0x2b8
	pub PingPongModule: *const c_void,	// 0x2c0
	pub ChessRogueModule: *const c_void,	// 0x2c8
	pub ActivityGuessTheSilhouetteModule: *const c_void,	// 0x2d0
	pub MapRotationModule: *const c_void,	// 0x2d8
	pub TreasureDungeonModule: *const c_void,	// 0x2e0
	pub FloorConnectivityModule: *const c_void,	// 0x2e8
	pub PetModule: *const c_void,	// 0x2f0
	pub BoxingClubModule: *const c_void,	// 0x2f8
	pub QuestModule: *const c_void,	// 0x300
	pub OfferingModule: *const c_void,	// 0x308
	pub AvatarModule: *const c_void,	// 0x310
	pub GachaModule: *const c_void,	// 0x318
	pub ScheduleModule: *const c_void,	// 0x320
	pub PayModule: *const c_void,	// 0x328
	pub ChimeraModule: *const c_void,	// 0x330
	pub LuaDataModule: *const c_void,	// 0x338
	pub PamModule: *const c_void,	// 0x340
	pub TrainModule: *const c_void,	// 0x348
	pub ActivityPlayerReturnModule: *const c_void,	// 0x350
	pub ActivityStrongChallengeModule: *const c_void,	// 0x358
	pub InventoryModule: *const c_void,	// 0x360
	pub ActivityBenefitModule: *const c_void,	// 0x368
	pub FightActivityModule: *const c_void,	// 0x370
	pub GrowthModule: *const c_void,	// 0x378
	pub ActivityAetherDivideModule: *const c_void,	// 0x380
	pub StoryTokenModule: *const c_void,	// 0x388
	pub HeartDialModule: *const c_void,	// 0x390
	pub ColonyCollectionPuzzleModule: *const c_void,	// 0x398
	pub BattlePassModule: *const c_void,	// 0x3a0
	pub AnniversaryCollectionModule: *const c_void,	// 0x3a8
	pub ActivityPhotoExhibitionModule: *const c_void,	// 0x3b0
	pub RogueMagicModule: *const c_void,	// 0x3b8
	pub BattleEventModule: *const c_void,	// 0x3c0
	pub PamSkinModule: *const c_void,	// 0x3c8
	pub ActivityParkourModule: *const c_void,	// 0x3d0
	pub ActivityClockParkModule: *const c_void,	// 0x3d8
	pub modules: *const NativeArray<NativeObject>,	// 0x3e0
	pub PunkLordModule: *const c_void,	// 0x3e8
	pub NavMapModule: *const c_void,	// 0x3f0
	pub StoryLineModule: *const c_void,	// 0x3f8
	pub MissionTimelineModule: *const c_void,	// 0x400
	pub TextJoinModule: *const c_void,	// 0x408
	pub RogueModule: *const c_void,	// 0x410
	pub _ModuleInitRequestList: *const NativeArray<NativeObject>,	// 0x418
	pub TarotBookModule: *const c_void,	// 0x420
	pub AlleyModule: *const c_void,	// 0x428
	pub ActivityTelevisionModule: *const c_void,	// 0x430
	pub RaidCollectionModule: *const c_void,	// 0x438
	pub LoadingTipsModule: *const c_void,	// 0x440
	pub WhiteListInteractUploadModule: *const c_void,	// 0x448
	pub RaidModule: *const c_void,	// 0x450
	pub SpaceZooModule: *const c_void,	// 0x458
	pub EntityTimeRewindModule: *const c_void,	// 0x460
	pub RogueAdventureModule: *const c_void,	// 0x468
	pub AchievementModule: *const c_void,	// 0x470
	pub MultiplayerGameModule: *const c_void,	// 0x478
	pub FriendModule: *const c_void,	// 0x480
	pub MissionChronicleModule: *const c_void,	// 0x488
	pub RogueHandbookModule: *const c_void,	// 0x490
	pub EraFlipperModule: *const c_void,	// 0x498
	pub ArchiveModule: *const c_void,	// 0x4a0
	pub GameStateServiceModule: *const c_void,	// 0x4a8
	pub MarbleModule: *const c_void,	// 0x4b0
	pub isInited: bool	// 0x4b8
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AvatarServantData {
	pub native_object: NativeObject,
	pub _SkillDataMap: *const c_void,	// 0x10
	pub _AvatarData: *const AvatarData,	// 0x18
	pub _Row: *const c_void,	// 0x20
	pub _ServantRowData: *const c_void,	// 0x28
	pub _Json: *const c_void	// 0x30
}
	}
	pub mod gamecore {
	use std::ffi::c_void;
	use crate::kreide::types::*;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BattleEventDataComponent {
	pub _parent_object:  CharacterDataComponent,	// 0x0
	pub _BattleEventRowData: *const c_void,	// 0x90
	pub BattleEventConfig__BackingField: *const c_void,	// 0x98
	pub SourceCaster__BackingField: *const GameEntity,	// 0xa0
	pub _EnergyBarState: *const c_void,	// 0xa8
	pub _TBAbilityRef: *const TurnBasedAbilityComponent,	// 0xb0
	pub CreateParams__BackingField: *const c_void,	// 0xb8
	pub WarningChallengeTurnLeft: u32,	// 0xc0
	pub BattleEventTotalDamageType: TeamType	// 0xc4
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GameEntity {
	pub native_object: NativeObject,
	pub TagComponentContainer: *const c_void,	// 0x10
	pub NameForGameCore__BackingField: *const NativeString,	// 0x18
	pub _DestroyWaitList: *const c_void,	// 0x20
	pub _ComponentList: *const c_void,	// 0x28
	pub _CurTickListRef: [u8; 0x10],	// 0x30
	pub _ComponentArray: *const NativeArray<GameComponentBase>,	// 0x40
	pub _TickLodProxy: *const c_void,	// 0x48
	pub OnTeamChange: *const c_void,	// 0x50
	pub TimeScaleStack: *const c_void,	// 0x58
	pub _UnstageReasonKey: *const NativeString,	// 0x60
	pub _ComponentArrayRef: *const c_void,	// 0x68
	pub _OwnerWorldRef: *const c_void,	// 0x70
	pub TickLodTemplate: *const NativeString,	// 0x78
	pub _LateUpdateComponentList: *const c_void,	// 0x80
	pub OnStageStateChange: *const c_void,	// 0x88
	pub _UnityGO: *const c_void,	// 0x90
	pub Name__BackingField: *const NativeString,	// 0x98
	pub WorldTimeScaleAdpator: *const c_void,	// 0xa0
	pub HoyoTagContainer: *const c_void,	// 0xa8
	pub DisposeCallback: *const c_void,	// 0xb0
	pub _TickComponentList: *const c_void,	// 0xb8
	pub LastTickTime__BackingField: f32,	// 0xc0
	pub _ServerEntityID: u32,	// 0xc4
	pub _GroupID: u32,	// 0xc8
	pub _TickDelayFrameCount: u32,	// 0xcc
	pub _EntityType: EntityType,	// 0xd0
	pub LastTickBucket__BackingField: i32,	// 0xd4
	pub CampID__BackingField: i32,	// 0xd8
	pub _GroupEntityID: u32,	// 0xdc
	pub _ForceTickLodLowestReason: *const c_void,	// 0xe0
	pub ForceIgnoreTickLodBistSet: u32,	// 0xe8
	pub RuntimeID__BackingField: u32,	// 0xec
	pub TickLodBoundSize__BackingField: f32,	// 0xf0
	pub _Team: TeamType,	// 0xf4
	pub HasDisposed: bool,	// 0xf8
	pub _ShouldLateUpdate: bool,	// 0xf9
	pub _IsOnStage: bool,	// 0xfa
	pub IsHero__BackingField: bool,	// 0xfb
	pub Visible__BackingField: bool,	// 0xfc
	pub IsLoaded__BackingField: bool,	// 0xfd
	pub IsStoryMode__BackingField: bool,	// 0xfe
	pub KillImmediately: bool,	// 0xff
	pub ObjectFeature__BackingField: i32,	// 0x100
	pub _AliveState: i32,	// 0x104
	pub _IsRegisterEnviroChara: bool,	// 0x108
	pub _Tickable: bool,	// 0x109
	pub IsFakeAvatar__BackingField: bool,	// 0x10a
	pub Disposing: bool,	// 0x10b
	pub LastTickFrame__BackingField: u64	// 0x110
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
pub struct SkillCharacterComponent {
	pub _parent_object:  GameComponentBase,	// 0x0
	pub CurrentAimAtMainTargetList: *const NativeArray<GameEntity>,	// 0x18
	pub AutoUseUltraParams: *const c_void,	// 0x20
	pub _recordAbilityInfo: [u8; 0x30],	// 0x28
	pub SkillActualAttacker__BackingField: *const GameEntity,	// 0x58
	pub CurrentAimAtTargetList: *const NativeArray<GameEntity>,	// 0x60
	pub TaskContext__BackingField: *const c_void,	// 0x68
	pub _CharacterDataRef: *const CharacterDataComponent,	// 0x70
	pub _SkillTypeDisableSlots: *const c_void,	// 0x78
	pub CurrentSkillTargetCharacterId: *const c_void,	// 0x80
	pub _JsonConfigRef: *const CharacterConfig,	// 0x88
	pub CurrentAimAtSubTargetList: *const NativeArray<GameEntity>,	// 0x90
	pub SkillPointEntity__BackingField: *const GameEntity,	// 0x98
	pub CurrentSkillTargetDamageHP: *const c_void,	// 0xa0
	pub CurrentSkillTargetList__BackingField: *const NativeArray<GameEntity>,	// 0xa8
	pub _SkillDataList: *const NativeArray<SkillData>,	// 0xb0
	pub _TBAbilityRef: *const TurnBasedAbilityComponent,	// 0xb8
	pub _SkillSlots: *const NativeArray<NativeObject>,	// 0xc0
	pub _SkillTypeDisableCountArr: *const NativeArray<i32>,	// 0xc8
	pub _SkillTargetRedirectEntries: *const NativeArray<NativeObject>,	// 0xd0
	pub CurrentSkillSubTargetList__BackingField: *const NativeArray<GameEntity>,	// 0xd8
	pub OnSkillSetup: *const NativeArray<NativeObject>,	// 0xe0
	pub _TargetPerformTimeCounter: f32,	// 0xe8
	pub _hasRecordSkill: bool,	// 0xec
	pub _isPassive: bool,	// 0xed
	pub CurrentSkillBreakStance: bool,	// 0xee
	pub _actionSkillIndex: i32,	// 0xf0
	pub _CurrentSkillExtraUseParam: i32,	// 0xf4
	pub _OpIndexInSkill: i32,	// 0xf8
	pub _CurrentSkillIndex: i32,	// 0xfc
	pub CurrentSkillKilledCount: i32,	// 0x100
	pub CurrentSkillKillAllOrBoss: bool,	// 0x104
	pub _AutoStandbyOnCurSkillFinish: bool,	// 0x105
	pub _RedirectTargetIDIncr: i32,	// 0x108
	pub _RecordSkillExtraUseParam: i32,	// 0x10c
	pub SelfWaitActiveSkillIndex: i32,	// 0x110
	pub _SelfSkillPerformState: i32,	// 0x114
	pub IsNoBpCost__BackingField: bool,	// 0x118
	pub CharmAction: bool,	// 0x119
	pub CurrentSkillHasTriggerEffect: bool,	// 0x11a
	pub _hasOpInSkill: bool	// 0x11b
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ServantSkillRowData {
	pub native_object: NativeObject,
	pub _Config: *const c_void,	// 0x10
	pub _OverrideData: [u8; 0xe8],	// 0x18
	pub _Row: *const c_void,	// 0x100
	pub _DefaultOverrideData: [u8; 0xe0]	// 0x108
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AvatarSkillRowData {
	pub native_object: NativeObject,
	pub _DefaultOverrideData: [u8; 0xe8],	// 0x10
	pub _OverrideData: [u8; 0xe8],	// 0xf8
	pub _Config: *const c_void,	// 0x1e0
	pub _Row: *const c_void	// 0x1e8
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FixPoint {
	pub m_rawValue: i64	// 0x0
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LineUpCharacter {
	pub native_object: NativeObject,
	pub BattleGridAvatarData: *const c_void,	// 0x10
	pub SkillTreePointList: *const NativeArray<NativeObject>,	// 0x18
	pub ChangedSkillTreePointList: *const NativeArray<NativeObject>,	// 0x20
	pub SpiritPassiveList: *const NativeArray<u32>,	// 0x28
	pub BattleRelicItemModule: *const c_void,	// 0x30
	pub BattleEquipmentList: *const NativeArray<NativeObject>,	// 0x38
	pub CharacterSP_Numerator: FixPoint,	// 0x40
	pub CharacterAvatarType: i32,	// 0x48
	pub CharacterPromotion: u32,	// 0x4c
	pub Index: u32,	// 0x50
	pub CharacterRowIndex: u32,	// 0x54
	pub CharacterRank: u32,	// 0x58
	pub TotalPower: u32,	// 0x5c
	pub CharacterSP_Denominator: FixPoint,	// 0x60
	pub CharacterHPRatio: FixPoint,	// 0x68
	pub SpiritLineupType: i32,	// 0x70
	pub WorldLevel: u32,	// 0x74
	pub AssistUid: u32,	// 0x78
	pub SpecialAvatarID: u32,	// 0x7c
	pub CharacterLevel: u32,	// 0x80
	pub CharacterID: u32	// 0x84
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
pub struct BattleEventSkillRowData {
	pub native_object: NativeObject,
	pub _Config: *const c_void,	// 0x10
	pub _DefaultOverrideData: [u8; 0xe8],	// 0x18
	pub _OverrideData: [u8; 0xe8],	// 0x100
	pub _Row: *const c_void	// 0x1e8
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
pub struct EntityManager {
	pub native_object: NativeObject,
	pub _UniqueNamedEntityDictionary: *const c_void,	// 0x10
	pub DataViewUISelectFadeOutSummonerEntity__BackingField: *const GameEntity,	// 0x18
	pub EntityGORoot__BackingField: *const c_void,	// 0x20
	pub GroupGORoot__BackingField: *const c_void,	// 0x28
	pub DataViewUILeaveSummonerOfUncreatedServant__BackingField: *const GameEntity,	// 0x30
	pub PerformanceGORoot__BackingField: *const c_void,	// 0x38
	pub DataViewUISelectFadeInEntity__BackingField: *const GameEntity,	// 0x40
	pub _AllTeamEntityList: *const NativeArray<GameEntity>,	// 0x48
	pub LittleGameGORoot__BackingField: *const c_void,	// 0x50
	pub LevelEntity__BackingField: *const GameEntity,	// 0x58
	pub _EntityUniqueNameDict: *const NativeArray<NativeObject>,	// 0x60
	pub _PauseEntityTimeSlowIndexDic: *const NativeArray<NativeObject>,	// 0x68
	pub _AllTeamEntity: *const NativeArray<GameEntity>,	// 0x70
	pub _ServerEntityIDToEntityDict: *const c_void,	// 0x78
	pub _AllEntityDictionary: *const c_void,	// 0x80
	pub _SnapshotEntityMap: *const c_void,	// 0x88
	pub PlayerGORoot__BackingField: *const c_void,	// 0x90
	pub _OwnerWorldRef: *const c_void,	// 0x98
	pub DataViewUISelectFadeInFollowEntities__BackingField: *const c_void,	// 0xa0
	pub DataViewUISelectFadeOutEntity__BackingField: *const GameEntity,	// 0xa8
	pub _ProcessEntityTeamChangeDelg: *const c_void,	// 0xb0
	pub _GroupEntityIDToEntityDict: *const c_void,	// 0xb8
	pub DataViewUISelectSummonerOfUncreatedServant__BackingField: *const GameEntity,	// 0xc0
	pub _UseUniqueSnapshot: bool	// 0xc8
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TurnBasedGameMode {
	pub native_object: NativeObject,
	pub _LimboEntitiesSkipSettlement: *const NativeArray<NativeObject>,	// 0x10
	pub DamageQueue__BackingField: *const c_void,	// 0x18
	pub _CurrentSkillCharacter: *const SkillCharacterComponent,	// 0x20
	pub _SwordTrainingMgr: *const c_void,	// 0x28
	pub LastSummonMonsterList: *const NativeArray<GameEntity>,	// 0x30
	pub _ImmediateActionEntities: *const c_void,	// 0x38
	pub _LinkTeammateList: *const NativeArray<GameEntity>,	// 0x40
	pub _TurnStateFSM: *const c_void,	// 0x48
	pub _VersusBarMgr: *const c_void,	// 0x50
	pub BattleCounter: *const c_void,	// 0x58
	pub _WaitingAbilityList: *const NativeArray<NativeObject>,	// 0x60
	pub _CachedDynamicSkillTargetSelection: *const GameEntity,	// 0x68
	pub _CurrentActionDelayModifyGroup: *const NativeArray<GameEntity>,	// 0x70
	pub _SomatoModifierPerforms: *const NativeArray<NativeObject>,	// 0x78
	pub _allowQuitStates: *const NativeArray<NativeObject>,	// 0x80
	pub ActionBarMgr__BackingField: *const c_void,	// 0x88
	pub _EntityCustomUnselectableDatas: *const NativeArray<NativeObject>,	// 0x90
	pub BattleChangeAvatarManager__BackingField: *const c_void,	// 0x98
	pub _InsertAbilityList: *const NativeArray<MMNDIEBMDNL>,	// 0xa0
	pub _SkillAddBuffPerformList: *const NativeArray<NativeObject>,	// 0xa8
	pub BattleEventInitedData__BackingField: *const c_void,	// 0xb0
	pub _CurrentTurnActionEntity: *const GameEntity,	// 0xb8
	pub LastKillTargetList__BackingField: *const NativeArray<GameEntity>,	// 0xc0
	pub LastTurnSnapshot: *const c_void,	// 0xc8
	pub _ReplayData: *const c_void,	// 0xd0
	pub CurrentTurnOwnerEntity__BackingField: *const GameEntity,	// 0xd8
	pub TimeGameStart: *const c_void,	// 0xe0
	pub ThisTurnAnimEvents: *const c_void,	// 0xe8
	pub MonsterWaveTextInfo: *const c_void,	// 0xf0
	pub _ActionEntityListSnapshot: *const NativeArray<GameEntity>,	// 0xf8
	pub _ModifierPerformCamerContext: *const c_void,	// 0x100
	pub LastZombie__BackingField: *const GameEntity,	// 0x108
	pub _AllTeamCharacters: *const NativeArray<GameEntity>,	// 0x110
	pub _LimboRevivableEntities: *const NativeArray<NativeObject>,	// 0x118
	pub _LimboEntities: *const NativeArray<NativeObject>,	// 0x120
	pub PrepareAbility__BackingField: *const c_void,	// 0x128
	pub _LimboEntitiesWaitAbilityFinish: *const NativeArray<NativeObject>,	// 0x130
	pub GridFightMananger__BackingField: *const c_void,	// 0x138
	pub _OverrieWaveMonsterPerformDatas: *const NativeArray<NativeObject>,	// 0x140
	pub _AttackingEntityList: *const c_void,	// 0x148
	pub _AidDetail: *const c_void,	// 0x150
	pub _ActionEntityList: *const NativeArray<GameEntity>,	// 0x158
	pub AssistantAvatarEntity__BackingField: *const GameEntity,	// 0x160
	pub _performParam: *const c_void,	// 0x168
	pub _InsertUltraSkillParamsQueue: *const NativeArray<NativeObject>,	// 0x170
	pub PerformDelayExecuteList: *const NativeArray<NativeObject>,	// 0x178
	pub _MainMonster: *const GameEntity,	// 0x180
	pub LastKillSkill__BackingField: *const c_void,	// 0x188
	pub _ActionDelayOrderTrigger: *const c_void,	// 0x190
	pub CurrentWaveMainMonsterIDPool__BackingField: *const NativeArray<u32>,	// 0x198
	pub _AllOffTeamCharacters: *const NativeArray<GameEntity>,	// 0x1a0
	pub _AvatarChangeParam: *const c_void,	// 0x1a8
	pub _LastBreakMonster: *const GameEntity,	// 0x1b0
	pub _ActionDelayChangeStamp: [u8; 0x18],	// 0x1b8
	pub OwnerBattleInstanceRef__BackingField: *const c_void,	// 0x1d0
	pub _RelationGroupMgr: *const c_void,	// 0x1d8
	pub _CurrentTurnTargetEntity: *const GameEntity,	// 0x1e0
	pub SkillUsageLog__BackingField: *const c_void,	// 0x1e8
	pub BattleAIPublicKnowledge__BackingField: *const c_void,	// 0x1f0
	pub CurrentMVPEntity__BackingField: *const GameEntity,	// 0x1f8
	pub _ActionDelayLinkMgr: *const c_void,	// 0x200
	pub _CommonSkillPoolNames: *const c_void,	// 0x208
	pub _UnselectableEntities: *const NativeArray<GameEntity>,	// 0x210
	pub _PhaseModifierList: *const NativeArray<NativeObject>,	// 0x218
	pub StageBattleEventMgr__BackingField: *const c_void,	// 0x220
	pub LastKillCaster__BackingField: *const GameEntity,	// 0x228
	pub _EntityModifierPerforms: *const c_void,	// 0x230
	pub TurnActionDelayCostChangeSource__BackingField: *const GameEntity,	// 0x238
	pub _RogueInBattleData: *const c_void,	// 0x240
	pub _CurModifierPerformSeq: *const c_void,	// 0x248
	pub _EventProcessor: *const c_void,	// 0x250
	pub _LevelLockedFeatureSet: *const c_void,	// 0x258
	pub _EvolveBuildGearMgr: *const c_void,	// 0x260
	pub _ModifierPerformTimeScale: f32,	// 0x268
	pub _ModifierEndingPerformRemainedTime: f32,	// 0x26c
	pub PropagateBeingAttackTeam__BackingField: TeamType,	// 0x270
	pub TurnOwnerPrepareAbilityUsed__BackingField: bool,	// 0x274
	pub LastKillFinish__BackingField: bool,	// 0x275
	pub _IsModifierPerformCameraSet: bool,	// 0x276
	pub ClearUltraSkillEffect: bool,	// 0x277
	pub _CurrentTurnTeam: TeamType,	// 0x278
	pub ChallengeTurnLimit__BackingField: u32,	// 0x27c
	pub _ModifierPerformTimerTotal: f32,	// 0x280
	pub BattleResultState__BackingField: *const c_void,	// 0x284
	pub PauseState__BackingField: i32,	// 0x28c
	pub TurnActionDelayCostRatio__BackingField: FixPoint,	// 0x290
	pub _ChallengeTurnAcc: u32,	// 0x298
	pub IsTeamFormationExpansion__BackingField: bool,	// 0x29c
	pub AutoInsertUltraSkill: bool,	// 0x29d
	pub _RequireMakeLimboEntitiesDie: bool,	// 0x29e
	pub IsActionOrder1UsedTBSkill__BackingField: bool,	// 0x29f
	pub _HitPerformMinTimer: f32,	// 0x2a0
	pub _PrevTickModeState: i32,	// 0x2a4
	pub _TurnCounter: u32,	// 0x2a8
	pub CurrentWaveIndexInStage__BackingField: u32,	// 0x2ac
	pub CurrentModeState__BackingField: i32,	// 0x2b0
	pub _ModifierPerformTimeTotal: f32,	// 0x2b4
	pub _LightTeamTurnCount: u32,	// 0x2b8
	pub _DamageCounter: u32,	// 0x2bc
	pub _ActionEntityListInited: bool,	// 0x2c0
	pub TurnOwnerActionPhaseEnd__BackingField: bool,	// 0x2c1
	pub IsLastKillTriggered: bool,	// 0x2c2
	pub LocalWinFlag__BackingField: [u8; 0x2],	// 0x2c3
	pub CertainlyWinInAdvance__BackingField: bool,	// 0x2c5
	pub _AutoBattle: bool,	// 0x2c6
	pub _WaveMonsterCurrentCount: i32,	// 0x2c8
	pub UseSkillOneMoreDefaultSkill: i32,	// 0x2cc
	pub WaveMonsterMaxCount__BackingField: i32,	// 0x2d0
	pub WinFlag: bool,	// 0x2d4
	pub CertainlyLoseInAdvance__BackingField: bool,	// 0x2d5
	pub PrepareAbilityFinish__BackingField: bool,	// 0x2d6
	pub _CurrentTurnActionEntitySkipActionFlag: bool,	// 0x2d7
	pub ChallengeTurnLimitType__BackingField: i32,	// 0x2d8
	pub _DarkTeamTurnCount: u32,	// 0x2dc
	pub ThisTurnAnimEventCount: i32,	// 0x2e0
	pub RealTimeCounter__BackingField: f32,	// 0x2e4
	pub _IsUseLinkSkill: bool,	// 0x2e8
	pub ApplyUIOperateOnDisableActionFlagChange: bool,	// 0x2e9
	pub ClearUltraSkillQueue__BackingField: bool,	// 0x2ea
	pub MuteLastKillTriggered: bool,	// 0x2eb
	pub CurrentInsertSkillSkipActionFlag: bool,	// 0x2ec
	pub IsManualExitBattle: bool,	// 0x2ed
	pub ApplyUIOperateOnSkillDisableChange: bool,	// 0x2ee
	pub _LastReplayAutoBattle: bool,	// 0x2ef
	pub _IsReplayBeingSaved: bool,	// 0x2f0
	pub AddOpCountOnInsertUltraWaitOrder: bool,	// 0x2f1
	pub PendingMonsterToWave__BackingField: bool,	// 0x2f2
	pub TurnEndKeep: bool,	// 0x2f3
	pub _RecordOperationByLG: *const c_void,	// 0x2f4
	pub _NextModifierIndex: i32,	// 0x2fc
	pub _OverrideAILocked: bool,	// 0x300
	pub IsManualRetryExitBattle: bool,	// 0x301
	pub _IsCreatingNewWave: bool,	// 0x302
	pub IsUseSkillOneMore: bool,	// 0x303
	pub ElapsedActionDelay__BackingField: FixPoint,	// 0x308
	pub SkipDeathHandle__BackingField: bool,	// 0x310
	pub SkipCameraDitherByLastKill: bool,	// 0x311
	pub _HoldFrameForCaptureFlag: bool,	// 0x312
	pub BattleResultAsWin: bool,	// 0x313
	pub BattleFinishReason: i32,	// 0x314
	pub _HoldFrameForCapture: u32,	// 0x318
	pub ShowCutinUIState__BackingField: i32,	// 0x31c
	pub _NextAbilityIndex: i32,	// 0x320
	pub SkipTurnOwnerActionFlag__BackingField: bool,	// 0x324
	pub ForbidAI: bool,	// 0x325
	pub _GamePauseFlag: bool,	// 0x326
	pub CurrentWaveStageID__BackingField: u32,	// 0x328
	pub _DeathVersion: u32,	// 0x32c
	pub _SkillExecutionEventState: i32,	// 0x330
	pub _OperationCounter: u32,	// 0x334
	pub _CachedDynamicSkillInput: i32,	// 0x338
	pub StanceCountDownSPChangeValue__BackingField: f32	// 0x33c
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CharacterDataComponent {
	pub _parent_object:  GameComponentBase,	// 0x0
	pub _DummpyEntityList: *const NativeArray<NativeObject>,	// 0x18
	pub _DynamicScaleAdaptEffectPathRule: *const c_void,	// 0x20
	pub _RowData: *const c_void,	// 0x28
	pub _DynamicScaleAdaptTypes: *const NativeArray<NativeObject>,	// 0x30
	pub _DynamicScaleAdaptConfigs: *const NativeArray<NativeObject>,	// 0x38
	pub Summoner: *const GameEntity,	// 0x40
	pub _CharacterUICustomValueDict: *const c_void,	// 0x48
	pub JsonConfig__BackingField: *const CharacterConfig,	// 0x50
	pub HideDisplayInfoSkillNames: *const c_void,	// 0x58
	pub EnhancedState: i32,	// 0x60
	pub LocalOffsetAsMoveTarget__BackingField: [u8; 0xc],	// 0x64
	pub GridFightTag__BackingField: i32,	// 0x70
	pub CreateReason: i32,	// 0x74
	pub IsVisibleInViewMode__BackingField: bool,	// 0x78
	pub _SaveModelWhenDeadOverride: [u8; 0x2],	// 0x79
	pub DisableRootYawMapping__BackingField: bool,	// 0x7b
	pub TriggerLimbo: bool,	// 0x7c
	pub IsBodyPart: bool,	// 0x7d
	pub DisableHeadLookAtActionEntityOverride: [u8; 0x2],	// 0x7e
	pub LastActTurnCount__BackingField: u32,	// 0x80
	pub SpawnTurnCount: u32,	// 0x84
	pub CharacterID__BackingField: u32,	// 0x88
	pub LineupIndex: i32	// 0x8c
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
pub struct SkillData {
	pub native_object: NativeObject,
	pub DefaultTargetInfo: *const c_void,	// 0x10
	pub Config: *const c_void,	// 0x18
	pub VisibleCondTask: *const c_void,	// 0x20
	pub ParentSkillData: *const SkillData,	// 0x28
	pub OverrideTargetInfo: *const c_void,	// 0x30
	pub OverrideCameraConfigAdded: *const c_void,	// 0x38
	pub OverrideCameraConfig: *const c_void,	// 0x40
	pub SkillTriggerKey: *const NativeString,	// 0x48
	pub InsertCondTask: *const c_void,	// 0x50
	pub CustomReadyConfigConditions: *const NativeArray<NativeObject>,	// 0x58
	pub _SkillProperties: *const NativeArray<NativeObject>,	// 0x60
	pub UsableCondTask: *const c_void,	// 0x68
	pub OverrideAnimState: *const NativeString,	// 0x70
	pub _Slot: *const c_void,	// 0x78
	pub AllChildSkillDatas: *const NativeArray<SkillData>,	// 0x80
	pub SkillCom: *const SkillCharacterComponent,	// 0x88
	pub PreshowConditions: *const NativeArray<NativeObject>,	// 0x90
	pub RowData: *const c_void,	// 0x98
	pub LeftCastTimes: i32,	// 0xa0
	pub CurrentCoolDown: i32,	// 0xa4
	pub MaxCastTimes: i32,	// 0xa8
	pub ChildIndex: i32,	// 0xac
	pub SkillIndex: i32,	// 0xb0
	pub AttackDamageTypePreshowAttach: i32,	// 0xb4
	pub SkillConfigID: u32,	// 0xb8
	pub DefaultCoolDown: i32,	// 0xbc
	pub CommonActiveSkillID: u32	// 0xc0
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TurnBasedAbilityComponent {
	pub _parent_object:  GameComponentBase,	// 0x0
	pub _ExtraMaxLayerConfig: *const NativeArray<NativeObject>,	// 0x18
	pub _AbilityToSkillMapping: *const c_void,	// 0x20
	pub _OnHitEffectOverride: *const NativeArray<NativeObject>,	// 0x28
	pub AbilityComponentRef__BackingField: *const c_void,	// 0x30
	pub _DmgChunk: *const c_void,	// 0x38
	pub _DelayModifyActionDelayQueue: *const c_void,	// 0x40
	pub _SyncPropertyMask: *const c_void,	// 0x48
	pub CharmDamageTarget: *const GameEntity,	// 0x50
	pub DamageDefender: *const GameEntity,	// 0x58
	pub _StatusProbabilityDict: *const c_void,	// 0x60
	pub OnAbilityPropertyChanged: *const NativeArray<NativeObject>,	// 0x68
	pub LockActionDelayChange: *const c_void,	// 0x70
	pub _JsonConfigRef: *const CharacterConfig,	// 0x78
	pub _RedStanceInfo: *const c_void,	// 0x80
	pub LastStanceBreakEntity__BackingField: *const GameEntity,	// 0x88
	pub KillerSkill__BackingField: *const c_void,	// 0x90
	pub _LockHPList: *const NativeArray<NativeObject>,	// 0x98
	pub ModifierOverrideMapping: *const c_void,	// 0xa0
	pub RegardAsSkillTypeMap: *const NativeArray<NativeObject>,	// 0xa8
	pub _StatusChanceResistanceDict: *const c_void,	// 0xb0
	pub OverflowStanceDamageAttacker__BackingField: *const GameEntity,	// 0xb8
	pub _TransformRef: *const c_void,	// 0xc0
	pub _AbilityProperties: *const NativeArray<NativeObject>,	// 0xc8
	pub _DamagedEntityListInAttack: *const NativeArray<GameEntity>,	// 0xd0
	pub CharmSkillName: *const NativeString,	// 0xd8
	pub _ModifierDelayParamList: *const c_void,	// 0xe0
	pub AdditionalAbilityParamList: *const NativeArray<NativeObject>,	// 0xe8
	pub ProjectileTargetAttachPoint: *const NativeString,	// 0xf0
	pub _SelfExtrAbilityList: *const NativeArray<NativeString>,	// 0xf8
	pub Weakness: *const c_void,	// 0x100
	pub _LockShieldCounter: *const c_void,	// 0x108
	pub _KillerEntity: *const GameEntity,	// 0x110
	pub _DefaultStanceInfo: *const c_void,	// 0x118
	pub _CharacterDataRef: *const CharacterDataComponent,	// 0x120
	pub _AbilityPropertiesInitSnapshot: *const NativeArray<FixPoint>,	// 0x128
	pub _ModifierRecordList: *const c_void,	// 0x130
	pub _ModifierEventSourceMuteCounter: *const c_void,	// 0x138
	pub _ExtraStanceInfo: *const c_void,	// 0x140
	pub _EnableNegativeHPSourceList: *const NativeArray<NativeObject>,	// 0x148
	pub _RedStanceInfoList: *const NativeArray<NativeObject>,	// 0x150
	pub ResistModifierBehaviorFlags__BackingField: *const NativeArray<NativeObject>,	// 0x158
	pub _OnHitEffectMultipleOverride: *const NativeArray<NativeObject>,	// 0x160
	pub CharmDamageAttackProperty: *const c_void,	// 0x168
	pub _SyncPropertySource: *const TurnBasedAbilityComponent,	// 0x170
	pub _DebuffLockStepSources: *const NativeArray<NativeObject>,	// 0x178
	pub _EnergyPointEntries: *const NativeArray<NativeObject>,	// 0x180
	pub _DamagedAllEntityIDListInAttack: *const c_void,	// 0x188
	pub RegardAsAttackTypeMap: *const NativeArray<NativeObject>,	// 0x190
	pub AddModifierBindValueMapping: *const c_void,	// 0x198
	pub CustomDataRef__BackingField: *const c_void,	// 0x1a0
	pub _DepartedParams: *const NativeArray<NativeObject>,	// 0x1a8
	pub _DotModifierEventProcessors: *const NativeArray<NativeObject>,	// 0x1b0
	pub _ModifierEventProcessors: *const NativeArray<NativeObject>,	// 0x1b8
	pub DamageSplitData: *const NativeArray<NativeObject>,	// 0x1c0
	pub _StancePreshowConfigs: *const NativeArray<NativeObject>,	// 0x1c8
	pub DisableActionStateByTask__BackingField: *const c_void,	// 0x1d0
	pub _BuffLockStepSources: *const NativeArray<NativeObject>,	// 0x1d8
	pub _DamageAttacker: *const GameEntity,	// 0x1e0
	pub _DamageStoreList: *const NativeArray<NativeObject>,	// 0x1e8
	pub TotalDamageCurrentAttack: FixPoint,	// 0x1f0
	pub _BuffLockStep: i32,	// 0x1f8
	pub CurrentAttackType__BackingField: AttackType,	// 0x1fc
	pub HasRevived: bool,	// 0x200
	pub _IsBehaviorFlagVisualDirty: bool,	// 0x201
	pub bIsInCharmAction: bool,	// 0x202
	pub BlockModifySp__BackingField: bool,	// 0x203
	pub _DeathVersion: u32,	// 0x204
	pub StanceState__BackingField: i32,	// 0x208
	pub IsSnapshot__BackingField: bool,	// 0x20c
	pub MuteTriggerDeath__BackingField: bool,	// 0x20d
	pub InsertAbilityCount: i32,	// 0x210
	pub ProjectileHitCount: i32,	// 0x214
	pub InheritSPRatio: FixPoint,	// 0x218
	pub BattleTag__BackingField: i32,	// 0x220
	pub LastStanceDamageType__BackingField: i32,	// 0x224
	pub _CurrentAttackPhase: i32,	// 0x228
	pub _ModifierUIOperationIncr: i32,	// 0x22c
	pub LastBreakStanceDamageType__BackingField: i32,	// 0x230
	pub _ModifierDelayAddCount: i32,	// 0x234
	pub MuteAllTriggerDeath__BackingField: bool,	// 0x238
	pub IsInAttack: bool,	// 0x239
	pub IsSharedDamageDataTarget: bool,	// 0x23a
	pub UseSpecialSP__BackingField: bool,	// 0x23b
	pub SpeedVisualFlagValue__BackingField: i32,	// 0x23c
	pub PropertyEnumBoundary__BackingField: i32,	// 0x240
	pub IsTriggeringStanceCountDown__BackingField: bool,	// 0x244
	pub CharmDisableBPAdd: bool,	// 0x245
	pub _BreakExtendEventUnsettled: bool,	// 0x246
	pub ForceKillFlag__BackingField: bool,	// 0x247
	pub _IsProcessingModifierDelayParam: bool,	// 0x248
	pub LockSelfActionDelay: bool,	// 0x249
	pub CharmDisableSPAdd: bool,	// 0x24a
	pub TriggerBreakExtendLogic: bool,	// 0x24b
	pub ActionDelayChanged__BackingField: [u8; 0x2],	// 0x24c
	pub IsTriggeredBlockDamage: bool,	// 0x24e
	pub PropertyChangeFlag__BackingField: bool,	// 0x24f
	pub ActionDelayDistance__BackingField: FixPoint,	// 0x250
	pub DeathSource__BackingField: i32,	// 0x258
	pub CharmDamageCount: i32,	// 0x25c
	pub StanceType: i32,	// 0x260
	pub _DebuffLockStep: i32,	// 0x264
	pub ForbidVisualFlagValue__BackingField: i32,	// 0x268
	pub VisualFlagValue__BackingField: i32,	// 0x26c
	pub OverflowStanceDamage__BackingField: FixPoint,	// 0x270
	pub _ResetStanceVersion: u32,	// 0x278
	pub _HighestPriorityOnHitEffect: i32,	// 0x27c
	pub TotalHitNum: FixPoint	// 0x280
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GameComponentBase {
	pub native_object: NativeObject,
	pub _OwnerRef: *const GameEntity	// 0x10
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BattleLineupData {
	pub native_object: NativeObject,
	pub _TemplateVariables: *const c_void,	// 0x10
	pub SpecialAvatarLevelAreaConfigs: *const c_void,	// 0x18
	pub AdditionalTemplateVariables: *const c_void,	// 0x20
	pub _LevelPath: *const NativeString,	// 0x28
	pub ExtraTeam: *const NativeArray<LineUpCharacter>,	// 0x30
	pub BattleExtraPropertyAdditionDict__BackingField: *const c_void,	// 0x38
	pub MazeBuffAdded: *const NativeArray<NativeObject>,	// 0x40
	pub LightTeam: *const NativeArray<LineUpCharacter>,	// 0x48
	pub DeferCreateTrialPlayerDic: *const c_void,	// 0x50
	pub Context: *const c_void,	// 0x58
	pub TeamBuffIDList: *const NativeArray<u32>,	// 0x60
	pub WorldLevel: u32	// 0x68
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
	}
}
