
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(rename_all = "camelCase")]
pub struct Standings {
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(rename = "_id")]
    pub id: Option<String>,

    pub games_played: Option<GamesPlayed>,

    #[serde(rename = "id")]
    pub standings_id: Option<String>,

    pub losses: Losses,

    pub runs: Option<Runs>,

    pub wins: Wins,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GamesPlayed {
    #[serde(rename = "045b4b38-fb11-4fa6-8dc0-f75997eacd28")]
    pub the_045_b4_b38_fb11_4_fa6_8_dc0_f75997_eacd28: Option<i64>,

    #[serde(rename = "0706f3cf-d6c4-4bd0-ac8c-de3d75ffa77e")]
    pub the_0706_f3_cf_d6_c4_4_bd0_ac8_c_de3_d75_ffa77_e: Option<i64>,

    #[serde(rename = "074b5e4a-84f8-428d-884e-4592a77ee061")]
    pub the_074_b5_e4_a_84_f8_428_d_884_e_4592_a77_ee061: Option<i64>,

    #[serde(rename = "09a77dd0-13c6-4c18-870a-63cd005ddff6")]
    pub the_09_a77_dd0_13_c6_4_c18_870_a_63_cd005_ddff6: Option<i64>,

    #[serde(rename = "0a449b4d-504b-448c-9516-6027fd6d216e")]
    pub the_0_a449_b4_d_504_b_448_c_95166027_fd6_d216_e: Option<i64>,

    #[serde(rename = "0b672007-ebfb-476d-8fdb-fb66bad78df2")]
    pub the_0_b672007_ebfb_476_d_8_fdb_fb66_bad78_df2: Option<i64>,

    #[serde(rename = "0eae657b-3592-43fb-93e1-b878680d2b53")]
    pub the_0_eae657_b_359243_fb_93_e1_b878680_d2_b53: Option<i64>,

    #[serde(rename = "105bc3ff-1320-4e37-8ef0-8d595cb95dd0")]
    pub the_105_bc3_ff_13204_e37_8_ef0_8_d595_cb95_dd0: Option<i64>,

    #[serde(rename = "110a62be-bc8a-4f0c-8046-0eb580b1af1c")]
    pub the_110_a62_be_bc8_a_4_f0_c_80460_eb580_b1_af1_c: Option<i64>,

    #[serde(rename = "11b92425-aa46-4691-b42f-baa2b6ddb541")]
    pub the_11_b92425_aa46_4691_b42_f_baa2_b6_ddb541: Option<i64>,

    #[serde(rename = "16be150e-372e-453e-b6ff-597aa42ca5ee")]
    pub the_16_be150_e_372_e_453_e_b6_ff_597_aa42_ca5_ee: Option<i64>,

    #[serde(rename = "16d1fd9b-c62b-4bed-b68a-b3a2d6e21524")]
    pub the_16_d1_fd9_b_c62_b_4_bed_b68_a_b3_a2_d6_e21524: Option<i64>,

    #[serde(rename = "19f81c84-9a94-49fa-9c23-2e1355126250")]
    pub the_19_f81_c84_9_a94_49_fa_9_c23_2_e1355126250: Option<i64>,

    #[serde(rename = "1a51664e-efec-45fa-b0ba-06d04c344628")]
    pub the_1_a51664_e_efec_45_fa_b0_ba_06_d04_c344628: Option<i64>,

    #[serde(rename = "1e04e5cc-80a6-41c0-af0d-7292817eed79")]
    pub the_1_e04_e5_cc_80_a6_41_c0_af0_d_7292817_eed79: Option<i64>,

    #[serde(rename = "22d8a1e9-e679-4bde-ae8a-318cb591d1c8")]
    pub the_22_d8_a1_e9_e679_4_bde_ae8_a_318_cb591_d1_c8: Option<i64>,

    #[serde(rename = "23a2cea4-5df7-4ed0-bb2c-b8c297518ada")]
    pub the_23_a2_cea4_5_df7_4_ed0_bb2_c_b8_c297518_ada: Option<i64>,

    #[serde(rename = "23e4cbc1-e9cd-47fa-a35b-bfa06f726cb7")]
    pub the_23_e4_cbc1_e9_cd_47_fa_a35_b_bfa06_f726_cb7: Option<i64>,

    #[serde(rename = "258f6389-aac1-43d2-b30a-4b4dde90d5eb")]
    pub the_258_f6389_aac1_43_d2_b30_a_4_b4_dde90_d5_eb: Option<i64>,

    #[serde(rename = "2957236a-6077-4012-a445-8c5be111afd0")]
    pub the_2957236_a_60774012_a445_8_c5_be111_afd0: Option<i64>,

    #[serde(rename = "2d02b60b-d858-4b8a-a835-c1e8fe1b8fe0")]
    pub the_2_d02_b60_b_d858_4_b8_a_a835_c1_e8_fe1_b8_fe0: Option<i64>,

    #[serde(rename = "2d07beca-bdb1-4ecb-bcfe-5913e2b406f5")]
    pub the_2_d07_beca_bdb1_4_ecb_bcfe_5913_e2_b406_f5: Option<i64>,

    #[serde(rename = "2dc7a1fa-3ae6-47ed-8c92-5d80167959f5")]
    pub the_2_dc7_a1_fa_3_ae6_47_ed_8_c92_5_d80167959_f5: Option<i64>,

    #[serde(rename = "2de5c38c-3d72-4d97-af0f-15e98eba2225")]
    pub the_2_de5_c38_c_3_d72_4_d97_af0_f_15_e98_eba2225: Option<i64>,

    #[serde(rename = "2e22beba-8e36-42ba-a8bf-975683c52b5f")]
    pub the_2_e22_beba_8_e36_42_ba_a8_bf_975683_c52_b5_f: Option<i64>,

    #[serde(rename = "2ec5927e-9905-408c-a04c-65a8879f846a")]
    pub the_2_ec5927_e_9905408_c_a04_c_65_a8879_f846_a: Option<i64>,

    #[serde(rename = "30c9bcd2-cc5a-421d-97d0-d39fefad053a")]
    pub the_30_c9_bcd2_cc5_a_421_d_97_d0_d39_fefad053_a: Option<i64>,

    #[serde(rename = "34a2e6ca-08fd-468e-894b-c707d6ce460a")]
    pub the_34_a2_e6_ca_08_fd_468_e_894_b_c707_d6_ce460_a: Option<i64>,

    #[serde(rename = "36569151-a2fb-43c1-9df7-2df512424c82")]
    pub the_36569151_a2_fb_43_c1_9_df7_2_df512424_c82: Option<i64>,

    #[serde(rename = "365b4517-4b0a-45da-aaa6-161dd77de99a")]
    pub the_365_b4517_4_b0_a_45_da_aaa6_161_dd77_de99_a: Option<i64>,

    #[serde(rename = "36f4efea-9d27-4457-a7b4-4b45ad2e23a3")]
    pub the_36_f4_efea_9_d27_4457_a7_b4_4_b45_ad2_e23_a3: Option<i64>,

    #[serde(rename = "378e3344-1a1a-4332-80cc-3da45954a4f4")]
    pub the_378_e3344_1_a1_a_433280_cc_3_da45954_a4_f4: Option<i64>,

    #[serde(rename = "3a094991-4cbc-4786-b74c-688876d243f4")]
    pub the_3_a094991_4_cbc_4786_b74_c_688876_d243_f4: Option<i64>,

    #[serde(rename = "3a4412d6-5404-4801-bf06-81cb7884fae4")]
    pub the_3_a4412_d6_54044801_bf06_81_cb7884_fae4: Option<i64>,

    #[serde(rename = "3aba36e6-9dd7-417f-9d3a-69d778439020")]
    pub the_3_aba36_e6_9_dd7_417_f_9_d3_a_69_d778439020: Option<i64>,

    #[serde(rename = "3b1c5a25-ed79-4ce2-87d4-0c1cf3ff342e")]
    pub the_3_b1_c5_a25_ed79_4_ce2_87_d4_0_c1_cf3_ff342_e: Option<i64>,

    #[serde(rename = "3cea1405-3a5f-432c-96c3-a85dc7f163ee")]
    pub the_3_cea1405_3_a5_f_432_c_96_c3_a85_dc7_f163_ee: Option<i64>,

    #[serde(rename = "3d858bda-dcef-4d05-928e-6557d3123f17")]
    pub the_3_d858_bda_dcef_4_d05_928_e_6557_d3123_f17: Option<i64>,

    #[serde(rename = "3f8bbb15-61c0-4e3f-8e4a-907a5fb1565e")]
    pub the_3_f8_bbb15_61_c0_4_e3_f_8_e4_a_907_a5_fb1565_e: Option<i64>,

    #[serde(rename = "444f2846-8927-4271-b2ca-6bf8b5b94610")]
    pub the_444_f2846_89274271_b2_ca_6_bf8_b5_b94610: Option<i64>,

    #[serde(rename = "44d9dc46-7e81-4e21-acff-c0f5dd399ae3")]
    pub the_44_d9_dc46_7_e81_4_e21_acff_c0_f5_dd399_ae3: Option<i64>,

    #[serde(rename = "46358869-dce9-4a01-bfba-ac24fc56f57e")]
    pub the_46358869_dce9_4_a01_bfba_ac24_fc56_f57_e: Option<i64>,

    #[serde(rename = "4b1004bc-345e-4084-8d18-b46315624864")]
    pub the_4_b1004_bc_345_e_40848_d18_b46315624864: Option<i64>,

    #[serde(rename = "4c192065-65d8-4010-8145-395f82d24ddf")]
    pub the_4_c192065_65_d8_40108145395_f82_d24_ddf: Option<i64>,

    #[serde(rename = "4cd14d96-f817-41a3-af6c-2d3ed0dd20b7")]
    pub the_4_cd14_d96_f817_41_a3_af6_c_2_d3_ed0_dd20_b7: Option<i64>,

    #[serde(rename = "4cf31f0e-fb42-4933-8fdb-fde58d109ced")]
    pub the_4_cf31_f0_e_fb42_49338_fdb_fde58_d109_ced: Option<i64>,

    #[serde(rename = "4d5f27b5-8924-498f-aa4c-7f5967c0c7c6")]
    pub the_4_d5_f27_b5_8924498_f_aa4_c_7_f5967_c0_c7_c6: Option<i64>,

    #[serde(rename = "505ae98b-7d85-4f51-99ef-60ccd7365d97")]
    pub the_505_ae98_b_7_d85_4_f51_99_ef_60_ccd7365_d97: Option<i64>,

    #[serde(rename = "5371833b-a620-4952-b2cb-a15eed8ad183")]
    pub the_5371833_b_a620_4952_b2_cb_a15_eed8_ad183: Option<i64>,

    #[serde(rename = "54d0d0f2-16e0-42a0-9fff-79cfa7c4a157")]
    pub the_54_d0_d0_f2_16_e0_42_a0_9_fff_79_cfa7_c4_a157: Option<i64>,

    #[serde(rename = "55c9fee3-79c8-4467-8dfb-ff1e340aae8c")]
    pub the_55_c9_fee3_79_c8_44678_dfb_ff1_e340_aae8_c: Option<i64>,

    #[serde(rename = "5663778c-c8fb-4408-908a-31dc1f6c55cc")]
    pub the_5663778_c_c8_fb_4408908_a_31_dc1_f6_c55_cc: Option<i64>,

    #[serde(rename = "5666fce7-3b39-4ade-9220-71244d9be5d8")]
    pub the_5666_fce7_3_b39_4_ade_922071244_d9_be5_d8: Option<i64>,

    #[serde(rename = "57d3f614-f8d3-4dfd-b486-075f823fdb0b")]
    pub the_57_d3_f614_f8_d3_4_dfd_b486_075_f823_fdb0_b: Option<i64>,

    #[serde(rename = "57ec08cc-0411-4643-b304-0e80dbc15ac7")]
    pub the_57_ec08_cc_04114643_b304_0_e80_dbc15_ac7: Option<i64>,

    #[serde(rename = "5818fb9b-f191-462e-9085-6fe311aaaf70")]
    pub the_5818_fb9_b_f191_462_e_90856_fe311_aaaf70: Option<i64>,

    #[serde(rename = "5d3bd8ab-cc9a-4aa5-bbcd-fa0f96566c64")]
    pub the_5_d3_bd8_ab_cc9_a_4_aa5_bbcd_fa0_f96566_c64: Option<i64>,

    #[serde(rename = "628bb28b-306e-4ff7-ad02-05524bcf246a")]
    pub the_628_bb28_b_306_e_4_ff7_ad02_05524_bcf246_a: Option<i64>,

    #[serde(rename = "635c332d-6ea9-4766-b391-ae4c3435f677")]
    pub the_635_c332_d_6_ea9_4766_b391_ae4_c3435_f677: Option<i64>,

    #[serde(rename = "6526d5df-6a9c-48e1-ba50-12dec0d8b22f")]
    pub the_6526_d5_df_6_a9_c_48_e1_ba50_12_dec0_d8_b22_f: Option<i64>,

    #[serde(rename = "6e655fc7-5190-4e55-99a0-89683d443cfc")]
    pub the_6_e655_fc7_51904_e55_99_a0_89683_d443_cfc: Option<i64>,

    #[serde(rename = "6f9ff34d-825f-477b-8600-1cec4febaecf")]
    pub the_6_f9_ff34_d_825_f_477_b_86001_cec4_febaecf: Option<i64>,

    #[serde(rename = "70167f08-5e85-44d7-b047-2961201c1615")]
    pub the_70167_f08_5_e85_44_d7_b047_2961201_c1615: Option<i64>,

    #[serde(rename = "71b157bc-8a50-4c05-a785-034f660e493f")]
    pub the_71_b157_bc_8_a50_4_c05_a785_034_f660_e493_f: Option<i64>,

    #[serde(rename = "747b8e4a-7e50-4638-a973-ea7950a3e739")]
    pub the_747_b8_e4_a_7_e50_4638_a973_ea7950_a3_e739: Option<i64>,

    #[serde(rename = "74aea6b6-34f9-48f4-b298-7345e1f9f7cb")]
    pub the_74_aea6_b6_34_f9_48_f4_b298_7345_e1_f9_f7_cb: Option<i64>,

    #[serde(rename = "75667373-b350-499b-b86e-5518b6f9f6ab")]
    pub the_75667373_b350_499_b_b86_e_5518_b6_f9_f6_ab: Option<i64>,

    #[serde(rename = "76d3489f-c7c4-4cb9-9c58-b1e1bab062d1")]
    pub the_76_d3489_f_c7_c4_4_cb9_9_c58_b1_e1_bab062_d1: Option<i64>,

    #[serde(rename = "780054a7-74ee-44fd-ab5f-6dd4637c5ef1")]
    pub the_780054_a7_74_ee_44_fd_ab5_f_6_dd4637_c5_ef1: Option<i64>,

    #[serde(rename = "79347640-8d5d-4e41-819d-2b0c86f20b76")]
    pub the_793476408_d5_d_4_e41_819_d_2_b0_c86_f20_b76: Option<i64>,

    #[serde(rename = "7966eb04-efcc-499b-8f03-d13916330531")]
    pub the_7966_eb04_efcc_499_b_8_f03_d13916330531: Option<i64>,

    #[serde(rename = "7ce9e0b0-9639-45f1-8db6-32c30ca0012d")]
    pub the_7_ce9_e0_b0_963945_f1_8_db6_32_c30_ca0012_d: Option<i64>,

    #[serde(rename = "7dc37924-0bb8-4e40-a826-c497d51e447c")]
    pub the_7_dc37924_0_bb8_4_e40_a826_c497_d51_e447_c: Option<i64>,

    #[serde(rename = "86435ed2-d205-4c37-be22-89683b9a7a62")]
    pub the_86435_ed2_d205_4_c37_be22_89683_b9_a7_a62: Option<i64>,

    #[serde(rename = "86f4485a-a6db-470b-82f5-e95e6b353537")]
    pub the_86_f4485_a_a6_db_470_b_82_f5_e95_e6_b353537: Option<i64>,

    #[serde(rename = "8756d8e1-bd9d-4116-8fd0-5ea06c2e80c3")]
    pub the_8756_d8_e1_bd9_d_41168_fd0_5_ea06_c2_e80_c3: Option<i64>,

    #[serde(rename = "878c1bf6-0d21-4659-bfee-916c8314d69c")]
    pub the_878_c1_bf6_0_d21_4659_bfee_916_c8314_d69_c: Option<i64>,

    #[serde(rename = "88151292-6c12-4fb8-b2d6-3e64821293b3")]
    pub the_881512926_c12_4_fb8_b2_d6_3_e64821293_b3: Option<i64>,

    #[serde(rename = "89796ffb-843a-4163-8dec-1bef229c68cb")]
    pub the_89796_ffb_843_a_41638_dec_1_bef229_c68_cb: Option<i64>,

    #[serde(rename = "8981c839-cbcf-47e3-a74e-8731dcff24fe")]
    pub the_8981_c839_cbcf_47_e3_a74_e_8731_dcff24_fe: Option<i64>,

    #[serde(rename = "8aaf714b-d42a-40b0-9165-384befc66d55")]
    pub the_8_aaf714_b_d42_a_40_b0_9165384_befc66_d55: Option<i64>,

    #[serde(rename = "8b38afb3-2e20-4e73-bb00-22bab14e3cda")]
    pub the_8_b38_afb3_2_e20_4_e73_bb00_22_bab14_e3_cda: Option<i64>,

    #[serde(rename = "8d7ba290-5f87-403c-81e3-cf5a2b6a6082")]
    pub the_8_d7_ba290_5_f87_403_c_81_e3_cf5_a2_b6_a6082: Option<i64>,

    #[serde(rename = "8d87c468-699a-47a8-b40d-cfb73a5660ad")]
    pub the_8_d87_c468_699_a_47_a8_b40_d_cfb73_a5660_ad: Option<i64>,

    #[serde(rename = "910974d7-cbfd-4d2d-8733-7e85759932da")]
    pub the_910974_d7_cbfd_4_d2_d_87337_e85759932_da: Option<i64>,

    #[serde(rename = "93e71a0e-80fc-46b7-beaf-d204c425fe03")]
    pub the_93_e71_a0_e_80_fc_46_b7_beaf_d204_c425_fe03: Option<i64>,

    #[serde(rename = "93f91157-f628-4c9a-a392-d2b1dbd79ac5")]
    pub the_93_f91157_f628_4_c9_a_a392_d2_b1_dbd79_ac5: Option<i64>,

    #[serde(rename = "9494152b-99f6-4adb-9573-f9e084bc813f")]
    pub the_9494152_b_99_f6_4_adb_9573_f9_e084_bc813_f: Option<i64>,

    #[serde(rename = "9657f04e-1c51-4951-88de-d376bb57f5bd")]
    pub the_9657_f04_e_1_c51_495188_de_d376_bb57_f5_bd: Option<i64>,

    #[serde(rename = "9685b9a9-8765-49e1-88ca-c153ad0276d0")]
    pub the_9685_b9_a9_876549_e1_88_ca_c153_ad0276_d0: Option<i64>,

    #[serde(rename = "979aee4a-6d80-4863-bf1c-ee1a78e06024")]
    pub the_979_aee4_a_6_d80_4863_bf1_c_ee1_a78_e06024: Option<i64>,

    #[serde(rename = "99edf2f4-f47d-46ee-998a-4cb4200236f7")]
    pub the_99_edf2_f4_f47_d_46_ee_998_a_4_cb4200236_f7: Option<i64>,

    #[serde(rename = "9a2f6bb9-c72c-437c-a3c4-e076dc5d10d4")]
    pub the_9_a2_f6_bb9_c72_c_437_c_a3_c4_e076_dc5_d10_d4: Option<i64>,

    #[serde(rename = "9da5c6b8-ccad-4bb5-b6b8-dc1d6b8ca6ed")]
    pub the_9_da5_c6_b8_ccad_4_bb5_b6_b8_dc1_d6_b8_ca6_ed: Option<i64>,

    #[serde(rename = "9debc64f-74b7-4ae1-a4d6-fce0144b6ea5")]
    pub the_9_debc64_f_74_b7_4_ae1_a4_d6_fce0144_b6_ea5: Option<i64>,

    #[serde(rename = "a01f0ade-0186-464d-8c68-e19a29cb66f0")]
    pub a01_f0_ade_0186464_d_8_c68_e19_a29_cb66_f0: Option<i64>,

    #[serde(rename = "a37f9158-7f82-46bc-908c-c9e2dda7c33b")]
    pub a37_f9158_7_f82_46_bc_908_c_c9_e2_dda7_c33_b: Option<i64>,

    #[serde(rename = "a554e084-b483-42da-89fb-39cd49ad7df6")]
    pub a554_e084_b483_42_da_89_fb_39_cd49_ad7_df6: Option<i64>,

    #[serde(rename = "a922603d-30c6-48d1-a83b-ae9be96675b6")]
    pub a922603_d_30_c6_48_d1_a83_b_ae9_be96675_b6: Option<i64>,

    #[serde(rename = "a94de6ef-5fc9-4470-89a0-557072fe4daf")]
    pub a94_de6_ef_5_fc9_447089_a0_557072_fe4_daf: Option<i64>,

    #[serde(rename = "adc5b394-8f76-416d-9ce9-813706877b84")]
    pub adc5_b394_8_f76_416_d_9_ce9_813706877_b84: Option<i64>,

    #[serde(rename = "ae0661b9-af66-4d4b-acc7-041e5cccb4bb")]
    pub ae0661_b9_af66_4_d4_b_acc7_041_e5_cccb4_bb: Option<i64>,

    #[serde(rename = "b024e975-1c4a-4575-8936-a3754a08806a")]
    pub b024_e975_1_c4_a_45758936_a3754_a08806_a: Option<i64>,

    #[serde(rename = "b069fdc6-2204-423a-932c-09037adcd845")]
    pub b069_fdc6_2204423_a_932_c_09037_adcd845: Option<i64>,

    #[serde(rename = "b1a50aa9-c515-46e8-8db9-d5378840362c")]
    pub b1_a50_aa9_c515_46_e8_8_db9_d5378840362_c: Option<i64>,

    #[serde(rename = "b320131f-da0d-43e1-9b98-f936a0ee417a")]
    pub b320131_f_da0_d_43_e1_9_b98_f936_a0_ee417_a: Option<i64>,

    #[serde(rename = "b35926d4-22a3-4419-8fab-686c41687055")]
    pub b35926_d4_22_a3_44198_fab_686_c41687055: Option<i64>,

    #[serde(rename = "b47df036-3aa4-4b98-8e9e-fe1d3ff1894b")]
    pub b47_df036_3_aa4_4_b98_8_e9_e_fe1_d3_ff1894_b: Option<i64>,

    #[serde(rename = "b5b4fb6b-08d8-401a-85d5-f08afa84af63")]
    pub b5_b4_fb6_b_08_d8_401_a_85_d5_f08_afa84_af63: Option<i64>,

    #[serde(rename = "b63be8c2-576a-4d6e-8daf-814f8bcea96f")]
    pub b63_be8_c2_576_a_4_d6_e_8_daf_814_f8_bcea96_f: Option<i64>,

    #[serde(rename = "b6b5df8f-5602-4883-b47d-07e77ed9d5af")]
    pub b6_b5_df8_f_56024883_b47_d_07_e77_ed9_d5_af: Option<i64>,

    #[serde(rename = "b72f3061-f573-40d7-832a-5ad475bd7909")]
    pub b72_f3061_f573_40_d7_832_a_5_ad475_bd7909: Option<i64>,

    #[serde(rename = "b7df2ea6-f4e8-4e6b-8c98-f730701f3717")]
    pub b7_df2_ea6_f4_e8_4_e6_b_8_c98_f730701_f3717: Option<i64>,

    #[serde(rename = "b7f9cc0c-6a6c-4bed-adbb-2d2d2dfbe810")]
    pub b7_f9_cc0_c_6_a6_c_4_bed_adbb_2_d2_d2_dfbe810: Option<i64>,

    #[serde(rename = "ba6d5599-1242-41ed-be64-90de7b1c255f")]
    pub ba6_d5599_124241_ed_be64_90_de7_b1_c255_f: Option<i64>,

    #[serde(rename = "bb4a9de5-c924-4923-a0cb-9d1445f1ee5d")]
    pub bb4_a9_de5_c924_4923_a0_cb_9_d1445_f1_ee5_d: Option<i64>,

    #[serde(rename = "bebf13f9-82d1-4133-9b14-4a96de029ccf")]
    pub bebf13_f9_82_d1_41339_b14_4_a96_de029_ccf: Option<i64>,

    #[serde(rename = "bfd38797-8404-4b38-8b82-341da28b1f83")]
    pub bfd38797_84044_b38_8_b82_341_da28_b1_f83: Option<i64>,

    #[serde(rename = "c0dc2c80-463e-49f7-9e00-c62473d677c8")]
    pub c0_dc2_c80_463_e_49_f7_9_e00_c62473_d677_c8: Option<i64>,

    #[serde(rename = "c19bb50b-9a22-4dd2-8200-bce639b1b239")]
    pub c19_bb50_b_9_a22_4_dd2_8200_bce639_b1_b239: Option<i64>,

    #[serde(rename = "c73b705c-40ad-4633-a6ed-d357ee2e2bcf")]
    pub c73_b705_c_40_ad_4633_a6_ed_d357_ee2_e2_bcf: Option<i64>,

    #[serde(rename = "c794d5aa-6104-420e-ae6f-3b2c270253fd")]
    pub c794_d5_aa_6104420_e_ae6_f_3_b2_c270253_fd: Option<i64>,

    #[serde(rename = "ca117809-cda1-4ae0-b607-53079fb5b133")]
    pub ca117809_cda1_4_ae0_b607_53079_fb5_b133: Option<i64>,

    #[serde(rename = "ca3f1c8c-c025-4d8e-8eef-5be6accbeb16")]
    pub ca3_f1_c8_c_c025_4_d8_e_8_eef_5_be6_accbeb16: Option<i64>,

    #[serde(rename = "cade1731-39a8-43f3-be8e-d2302711fe8b")]
    pub cade1731_39_a8_43_f3_be8_e_d2302711_fe8_b: Option<i64>,

    #[serde(rename = "cbd44c06-231a-4d1a-bb7d-4170b06e566a")]
    pub cbd44_c06_231_a_4_d1_a_bb7_d_4170_b06_e566_a: Option<i64>,

    #[serde(rename = "cc9de838-4431-4cc7-9c3e-15b15b2142b0")]
    pub cc9_de838_44314_cc7_9_c3_e_15_b15_b2142_b0: Option<i64>,

    #[serde(rename = "cd29d13d-99d4-414b-8faa-f0819b2de526")]
    pub cd29_d13_d_99_d4_414_b_8_faa_f0819_b2_de526: Option<i64>,

    #[serde(rename = "d0762a7e-004b-48a9-a832-a993982b305b")]
    pub d0762_a7_e_004_b_48_a9_a832_a993982_b305_b: Option<i64>,

    #[serde(rename = "d2874e7f-8e88-442a-a176-e256df68a49b")]
    pub d2874_e7_f_8_e88_442_a_a176_e256_df68_a49_b: Option<i64>,

    #[serde(rename = "d2949bd0-6a28-4e0d-aa07-cecc437cbd99")]
    pub d2949_bd0_6_a28_4_e0_d_aa07_cecc437_cbd99: Option<i64>,

    #[serde(rename = "d2c33336-b5a9-4ce1-86bb-f376ec66efbd")]
    pub d2_c33336_b5_a9_4_ce1_86_bb_f376_ec66_efbd: Option<i64>,

    #[serde(rename = "d6a352fc-b675-40a0-864d-f4fd50aaeea0")]
    pub d6_a352_fc_b675_40_a0_864_d_f4_fd50_aaeea0: Option<i64>,

    #[serde(rename = "d82a1a80-dff3-4767-bab6-484b2eb7aee1")]
    pub d82_a1_a80_dff3_4767_bab6_484_b2_eb7_aee1: Option<i64>,

    #[serde(rename = "d9f89a8a-c563-493e-9d64-78e4f9a55d4a")]
    pub d9_f89_a8_a_c563_493_e_9_d64_78_e4_f9_a55_d4_a: Option<i64>,

    #[serde(rename = "e11df0cc-3a95-4159-9a84-fecbbf23ae05")]
    pub e11_df0_cc_3_a95_41599_a84_fecbbf23_ae05: Option<i64>,

    #[serde(rename = "e12313fe-c0c9-49de-9d11-8b7408aa92ce")]
    pub e12313_fe_c0_c9_49_de_9_d11_8_b7408_aa92_ce: Option<i64>,

    #[serde(rename = "e4f7549c-17af-4e35-b89b-f0fae855a31b")]
    pub e4_f7549_c_17_af_4_e35_b89_b_f0_fae855_a31_b: Option<i64>,

    #[serde(rename = "ea3c8019-b6b6-4830-b952-7e9c2ce707bd")]
    pub ea3_c8019_b6_b6_4830_b952_7_e9_c2_ce707_bd: Option<i64>,

    #[serde(rename = "eb67ae5e-c4bf-46ca-bbbc-425cd34182ff")]
    pub eb67_ae5_e_c4_bf_46_ca_bbbc_425_cd34182_ff: Option<i64>,

    #[serde(rename = "ee722cbd-812f-4525-81d7-dfa89fb867a4")]
    pub ee722_cbd_812_f_452581_d7_dfa89_fb867_a4: Option<i64>,

    #[serde(rename = "effdbd8d-a54f-4049-a3c8-b5f944e5278b")]
    pub effdbd8_d_a54_f_4049_a3_c8_b5_f944_e5278_b: Option<i64>,

    #[serde(rename = "f02aeae2-5e6a-4098-9842-02d2273f25c7")]
    pub f02_aeae2_5_e6_a_4098984202_d2273_f25_c7: Option<i64>,

    #[serde(rename = "f0ec8435-0427-4ffd-ad0c-a67f60a75e0e")]
    pub f0_ec8435_04274_ffd_ad0_c_a67_f60_a75_e0_e: Option<i64>,

    #[serde(rename = "f3490435-a42f-42a8-ab89-d59e8dc8d599")]
    pub f3490435_a42_f_42_a8_ab89_d59_e8_dc8_d599: Option<i64>,

    #[serde(rename = "f8d99dc7-ae37-4f35-b08c-543864a347f2")]
    pub f8_d99_dc7_ae37_4_f35_b08_c_543864_a347_f2: Option<i64>,

    #[serde(rename = "f9045b82-5570-43d4-856b-bed5095515c6")]
    pub f9045_b82_557043_d4_856_b_bed5095515_c6: Option<i64>,

    #[serde(rename = "fab9420f-0730-4054-bd17-355113f204c2")]
    pub fab9420_f_07304054_bd17_355113_f204_c2: Option<i64>,

    #[serde(rename = "fca16c92-5f03-45b9-abbe-760866878ffe")]
    pub fca16_c92_5_f03_45_b9_abbe_760866878_ffe: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Losses {
    #[serde(rename = "045b4b38-fb11-4fa6-8dc0-f75997eacd28")]
    pub the_045_b4_b38_fb11_4_fa6_8_dc0_f75997_eacd28: Option<i64>,

    #[serde(rename = "0706f3cf-d6c4-4bd0-ac8c-de3d75ffa77e")]
    pub the_0706_f3_cf_d6_c4_4_bd0_ac8_c_de3_d75_ffa77_e: Option<i64>,

    #[serde(rename = "074b5e4a-84f8-428d-884e-4592a77ee061")]
    pub the_074_b5_e4_a_84_f8_428_d_884_e_4592_a77_ee061: Option<i64>,

    #[serde(rename = "09a77dd0-13c6-4c18-870a-63cd005ddff6")]
    pub the_09_a77_dd0_13_c6_4_c18_870_a_63_cd005_ddff6: Option<i64>,

    #[serde(rename = "0a449b4d-504b-448c-9516-6027fd6d216e")]
    pub the_0_a449_b4_d_504_b_448_c_95166027_fd6_d216_e: Option<i64>,

    #[serde(rename = "0b672007-ebfb-476d-8fdb-fb66bad78df2")]
    pub the_0_b672007_ebfb_476_d_8_fdb_fb66_bad78_df2: Option<i64>,

    #[serde(rename = "0eae657b-3592-43fb-93e1-b878680d2b53")]
    pub the_0_eae657_b_359243_fb_93_e1_b878680_d2_b53: Option<i64>,

    #[serde(rename = "105bc3ff-1320-4e37-8ef0-8d595cb95dd0")]
    pub the_105_bc3_ff_13204_e37_8_ef0_8_d595_cb95_dd0: Option<i64>,

    #[serde(rename = "110a62be-bc8a-4f0c-8046-0eb580b1af1c")]
    pub the_110_a62_be_bc8_a_4_f0_c_80460_eb580_b1_af1_c: Option<i64>,

    #[serde(rename = "11b92425-aa46-4691-b42f-baa2b6ddb541")]
    pub the_11_b92425_aa46_4691_b42_f_baa2_b6_ddb541: Option<i64>,

    #[serde(rename = "16be150e-372e-453e-b6ff-597aa42ca5ee")]
    pub the_16_be150_e_372_e_453_e_b6_ff_597_aa42_ca5_ee: Option<i64>,

    #[serde(rename = "16d1fd9b-c62b-4bed-b68a-b3a2d6e21524")]
    pub the_16_d1_fd9_b_c62_b_4_bed_b68_a_b3_a2_d6_e21524: Option<i64>,

    #[serde(rename = "19f81c84-9a94-49fa-9c23-2e1355126250")]
    pub the_19_f81_c84_9_a94_49_fa_9_c23_2_e1355126250: Option<i64>,

    #[serde(rename = "1a51664e-efec-45fa-b0ba-06d04c344628")]
    pub the_1_a51664_e_efec_45_fa_b0_ba_06_d04_c344628: Option<i64>,

    #[serde(rename = "1e04e5cc-80a6-41c0-af0d-7292817eed79")]
    pub the_1_e04_e5_cc_80_a6_41_c0_af0_d_7292817_eed79: Option<i64>,

    #[serde(rename = "22d8a1e9-e679-4bde-ae8a-318cb591d1c8")]
    pub the_22_d8_a1_e9_e679_4_bde_ae8_a_318_cb591_d1_c8: Option<i64>,

    #[serde(rename = "23a2cea4-5df7-4ed0-bb2c-b8c297518ada")]
    pub the_23_a2_cea4_5_df7_4_ed0_bb2_c_b8_c297518_ada: Option<i64>,

    #[serde(rename = "23e4cbc1-e9cd-47fa-a35b-bfa06f726cb7")]
    pub the_23_e4_cbc1_e9_cd_47_fa_a35_b_bfa06_f726_cb7: Option<i64>,

    #[serde(rename = "258f6389-aac1-43d2-b30a-4b4dde90d5eb")]
    pub the_258_f6389_aac1_43_d2_b30_a_4_b4_dde90_d5_eb: Option<i64>,

    #[serde(rename = "2957236a-6077-4012-a445-8c5be111afd0")]
    pub the_2957236_a_60774012_a445_8_c5_be111_afd0: Option<i64>,

    #[serde(rename = "2d02b60b-d858-4b8a-a835-c1e8fe1b8fe0")]
    pub the_2_d02_b60_b_d858_4_b8_a_a835_c1_e8_fe1_b8_fe0: Option<i64>,

    #[serde(rename = "2d07beca-bdb1-4ecb-bcfe-5913e2b406f5")]
    pub the_2_d07_beca_bdb1_4_ecb_bcfe_5913_e2_b406_f5: Option<i64>,

    #[serde(rename = "2dc7a1fa-3ae6-47ed-8c92-5d80167959f5")]
    pub the_2_dc7_a1_fa_3_ae6_47_ed_8_c92_5_d80167959_f5: Option<i64>,

    #[serde(rename = "2de5c38c-3d72-4d97-af0f-15e98eba2225")]
    pub the_2_de5_c38_c_3_d72_4_d97_af0_f_15_e98_eba2225: Option<i64>,

    #[serde(rename = "2e22beba-8e36-42ba-a8bf-975683c52b5f")]
    pub the_2_e22_beba_8_e36_42_ba_a8_bf_975683_c52_b5_f: Option<i64>,

    #[serde(rename = "2ec5927e-9905-408c-a04c-65a8879f846a")]
    pub the_2_ec5927_e_9905408_c_a04_c_65_a8879_f846_a: Option<i64>,

    #[serde(rename = "30c9bcd2-cc5a-421d-97d0-d39fefad053a")]
    pub the_30_c9_bcd2_cc5_a_421_d_97_d0_d39_fefad053_a: Option<i64>,

    #[serde(rename = "34a2e6ca-08fd-468e-894b-c707d6ce460a")]
    pub the_34_a2_e6_ca_08_fd_468_e_894_b_c707_d6_ce460_a: Option<i64>,

    #[serde(rename = "36569151-a2fb-43c1-9df7-2df512424c82")]
    pub the_36569151_a2_fb_43_c1_9_df7_2_df512424_c82: Option<i64>,

    #[serde(rename = "365b4517-4b0a-45da-aaa6-161dd77de99a")]
    pub the_365_b4517_4_b0_a_45_da_aaa6_161_dd77_de99_a: Option<i64>,

    #[serde(rename = "36f4efea-9d27-4457-a7b4-4b45ad2e23a3")]
    pub the_36_f4_efea_9_d27_4457_a7_b4_4_b45_ad2_e23_a3: Option<i64>,

    #[serde(rename = "378e3344-1a1a-4332-80cc-3da45954a4f4")]
    pub the_378_e3344_1_a1_a_433280_cc_3_da45954_a4_f4: Option<i64>,

    #[serde(rename = "3a094991-4cbc-4786-b74c-688876d243f4")]
    pub the_3_a094991_4_cbc_4786_b74_c_688876_d243_f4: Option<i64>,

    #[serde(rename = "3a4412d6-5404-4801-bf06-81cb7884fae4")]
    pub the_3_a4412_d6_54044801_bf06_81_cb7884_fae4: Option<i64>,

    #[serde(rename = "3aba36e6-9dd7-417f-9d3a-69d778439020")]
    pub the_3_aba36_e6_9_dd7_417_f_9_d3_a_69_d778439020: Option<i64>,

    #[serde(rename = "3b1c5a25-ed79-4ce2-87d4-0c1cf3ff342e")]
    pub the_3_b1_c5_a25_ed79_4_ce2_87_d4_0_c1_cf3_ff342_e: Option<i64>,

    #[serde(rename = "3cea1405-3a5f-432c-96c3-a85dc7f163ee")]
    pub the_3_cea1405_3_a5_f_432_c_96_c3_a85_dc7_f163_ee: Option<i64>,

    #[serde(rename = "3d858bda-dcef-4d05-928e-6557d3123f17")]
    pub the_3_d858_bda_dcef_4_d05_928_e_6557_d3123_f17: Option<i64>,

    #[serde(rename = "3f8bbb15-61c0-4e3f-8e4a-907a5fb1565e")]
    pub the_3_f8_bbb15_61_c0_4_e3_f_8_e4_a_907_a5_fb1565_e: Option<i64>,

    #[serde(rename = "444f2846-8927-4271-b2ca-6bf8b5b94610")]
    pub the_444_f2846_89274271_b2_ca_6_bf8_b5_b94610: Option<i64>,

    #[serde(rename = "44d9dc46-7e81-4e21-acff-c0f5dd399ae3")]
    pub the_44_d9_dc46_7_e81_4_e21_acff_c0_f5_dd399_ae3: Option<i64>,

    #[serde(rename = "46358869-dce9-4a01-bfba-ac24fc56f57e")]
    pub the_46358869_dce9_4_a01_bfba_ac24_fc56_f57_e: Option<i64>,

    #[serde(rename = "4b1004bc-345e-4084-8d18-b46315624864")]
    pub the_4_b1004_bc_345_e_40848_d18_b46315624864: Option<i64>,

    #[serde(rename = "4c192065-65d8-4010-8145-395f82d24ddf")]
    pub the_4_c192065_65_d8_40108145395_f82_d24_ddf: Option<i64>,

    #[serde(rename = "4cd14d96-f817-41a3-af6c-2d3ed0dd20b7")]
    pub the_4_cd14_d96_f817_41_a3_af6_c_2_d3_ed0_dd20_b7: Option<i64>,

    #[serde(rename = "4cf31f0e-fb42-4933-8fdb-fde58d109ced")]
    pub the_4_cf31_f0_e_fb42_49338_fdb_fde58_d109_ced: Option<i64>,

    #[serde(rename = "4d5f27b5-8924-498f-aa4c-7f5967c0c7c6")]
    pub the_4_d5_f27_b5_8924498_f_aa4_c_7_f5967_c0_c7_c6: Option<i64>,

    #[serde(rename = "505ae98b-7d85-4f51-99ef-60ccd7365d97")]
    pub the_505_ae98_b_7_d85_4_f51_99_ef_60_ccd7365_d97: Option<i64>,

    #[serde(rename = "5371833b-a620-4952-b2cb-a15eed8ad183")]
    pub the_5371833_b_a620_4952_b2_cb_a15_eed8_ad183: Option<i64>,

    #[serde(rename = "54d0d0f2-16e0-42a0-9fff-79cfa7c4a157")]
    pub the_54_d0_d0_f2_16_e0_42_a0_9_fff_79_cfa7_c4_a157: Option<i64>,

    #[serde(rename = "55c9fee3-79c8-4467-8dfb-ff1e340aae8c")]
    pub the_55_c9_fee3_79_c8_44678_dfb_ff1_e340_aae8_c: Option<i64>,

    #[serde(rename = "5663778c-c8fb-4408-908a-31dc1f6c55cc")]
    pub the_5663778_c_c8_fb_4408908_a_31_dc1_f6_c55_cc: Option<i64>,

    #[serde(rename = "5666fce7-3b39-4ade-9220-71244d9be5d8")]
    pub the_5666_fce7_3_b39_4_ade_922071244_d9_be5_d8: Option<i64>,

    #[serde(rename = "57d3f614-f8d3-4dfd-b486-075f823fdb0b")]
    pub the_57_d3_f614_f8_d3_4_dfd_b486_075_f823_fdb0_b: Option<i64>,

    #[serde(rename = "57ec08cc-0411-4643-b304-0e80dbc15ac7")]
    pub the_57_ec08_cc_04114643_b304_0_e80_dbc15_ac7: Option<i64>,

    #[serde(rename = "5818fb9b-f191-462e-9085-6fe311aaaf70")]
    pub the_5818_fb9_b_f191_462_e_90856_fe311_aaaf70: Option<i64>,

    #[serde(rename = "5d3bd8ab-cc9a-4aa5-bbcd-fa0f96566c64")]
    pub the_5_d3_bd8_ab_cc9_a_4_aa5_bbcd_fa0_f96566_c64: Option<i64>,

    #[serde(rename = "628bb28b-306e-4ff7-ad02-05524bcf246a")]
    pub the_628_bb28_b_306_e_4_ff7_ad02_05524_bcf246_a: Option<i64>,

    #[serde(rename = "635c332d-6ea9-4766-b391-ae4c3435f677")]
    pub the_635_c332_d_6_ea9_4766_b391_ae4_c3435_f677: Option<i64>,

    #[serde(rename = "6526d5df-6a9c-48e1-ba50-12dec0d8b22f")]
    pub the_6526_d5_df_6_a9_c_48_e1_ba50_12_dec0_d8_b22_f: Option<i64>,

    #[serde(rename = "6e655fc7-5190-4e55-99a0-89683d443cfc")]
    pub the_6_e655_fc7_51904_e55_99_a0_89683_d443_cfc: Option<i64>,

    #[serde(rename = "6f9ff34d-825f-477b-8600-1cec4febaecf")]
    pub the_6_f9_ff34_d_825_f_477_b_86001_cec4_febaecf: Option<i64>,

    #[serde(rename = "70167f08-5e85-44d7-b047-2961201c1615")]
    pub the_70167_f08_5_e85_44_d7_b047_2961201_c1615: Option<i64>,

    #[serde(rename = "71b157bc-8a50-4c05-a785-034f660e493f")]
    pub the_71_b157_bc_8_a50_4_c05_a785_034_f660_e493_f: Option<i64>,

    #[serde(rename = "747b8e4a-7e50-4638-a973-ea7950a3e739")]
    pub the_747_b8_e4_a_7_e50_4638_a973_ea7950_a3_e739: Option<i64>,

    #[serde(rename = "74aea6b6-34f9-48f4-b298-7345e1f9f7cb")]
    pub the_74_aea6_b6_34_f9_48_f4_b298_7345_e1_f9_f7_cb: Option<i64>,

    #[serde(rename = "75667373-b350-499b-b86e-5518b6f9f6ab")]
    pub the_75667373_b350_499_b_b86_e_5518_b6_f9_f6_ab: Option<i64>,

    #[serde(rename = "76d3489f-c7c4-4cb9-9c58-b1e1bab062d1")]
    pub the_76_d3489_f_c7_c4_4_cb9_9_c58_b1_e1_bab062_d1: Option<i64>,

    #[serde(rename = "780054a7-74ee-44fd-ab5f-6dd4637c5ef1")]
    pub the_780054_a7_74_ee_44_fd_ab5_f_6_dd4637_c5_ef1: Option<i64>,

    #[serde(rename = "79347640-8d5d-4e41-819d-2b0c86f20b76")]
    pub the_793476408_d5_d_4_e41_819_d_2_b0_c86_f20_b76: Option<i64>,

    #[serde(rename = "7966eb04-efcc-499b-8f03-d13916330531")]
    pub the_7966_eb04_efcc_499_b_8_f03_d13916330531: Option<i64>,

    #[serde(rename = "7ce9e0b0-9639-45f1-8db6-32c30ca0012d")]
    pub the_7_ce9_e0_b0_963945_f1_8_db6_32_c30_ca0012_d: Option<i64>,

    #[serde(rename = "7dc37924-0bb8-4e40-a826-c497d51e447c")]
    pub the_7_dc37924_0_bb8_4_e40_a826_c497_d51_e447_c: Option<i64>,

    #[serde(rename = "86435ed2-d205-4c37-be22-89683b9a7a62")]
    pub the_86435_ed2_d205_4_c37_be22_89683_b9_a7_a62: Option<i64>,

    #[serde(rename = "86f4485a-a6db-470b-82f5-e95e6b353537")]
    pub the_86_f4485_a_a6_db_470_b_82_f5_e95_e6_b353537: Option<i64>,

    #[serde(rename = "8756d8e1-bd9d-4116-8fd0-5ea06c2e80c3")]
    pub the_8756_d8_e1_bd9_d_41168_fd0_5_ea06_c2_e80_c3: Option<i64>,

    #[serde(rename = "878c1bf6-0d21-4659-bfee-916c8314d69c")]
    pub the_878_c1_bf6_0_d21_4659_bfee_916_c8314_d69_c: Option<i64>,

    #[serde(rename = "88151292-6c12-4fb8-b2d6-3e64821293b3")]
    pub the_881512926_c12_4_fb8_b2_d6_3_e64821293_b3: Option<i64>,

    #[serde(rename = "89796ffb-843a-4163-8dec-1bef229c68cb")]
    pub the_89796_ffb_843_a_41638_dec_1_bef229_c68_cb: Option<i64>,

    #[serde(rename = "8981c839-cbcf-47e3-a74e-8731dcff24fe")]
    pub the_8981_c839_cbcf_47_e3_a74_e_8731_dcff24_fe: Option<i64>,

    #[serde(rename = "8aaf714b-d42a-40b0-9165-384befc66d55")]
    pub the_8_aaf714_b_d42_a_40_b0_9165384_befc66_d55: Option<i64>,

    #[serde(rename = "8b38afb3-2e20-4e73-bb00-22bab14e3cda")]
    pub the_8_b38_afb3_2_e20_4_e73_bb00_22_bab14_e3_cda: Option<i64>,

    #[serde(rename = "8d7ba290-5f87-403c-81e3-cf5a2b6a6082")]
    pub the_8_d7_ba290_5_f87_403_c_81_e3_cf5_a2_b6_a6082: Option<i64>,

    #[serde(rename = "8d87c468-699a-47a8-b40d-cfb73a5660ad")]
    pub the_8_d87_c468_699_a_47_a8_b40_d_cfb73_a5660_ad: Option<i64>,

    #[serde(rename = "910974d7-cbfd-4d2d-8733-7e85759932da")]
    pub the_910974_d7_cbfd_4_d2_d_87337_e85759932_da: Option<i64>,

    #[serde(rename = "93e71a0e-80fc-46b7-beaf-d204c425fe03")]
    pub the_93_e71_a0_e_80_fc_46_b7_beaf_d204_c425_fe03: Option<i64>,

    #[serde(rename = "93f91157-f628-4c9a-a392-d2b1dbd79ac5")]
    pub the_93_f91157_f628_4_c9_a_a392_d2_b1_dbd79_ac5: Option<i64>,

    #[serde(rename = "9494152b-99f6-4adb-9573-f9e084bc813f")]
    pub the_9494152_b_99_f6_4_adb_9573_f9_e084_bc813_f: Option<i64>,

    #[serde(rename = "9657f04e-1c51-4951-88de-d376bb57f5bd")]
    pub the_9657_f04_e_1_c51_495188_de_d376_bb57_f5_bd: Option<i64>,

    #[serde(rename = "9685b9a9-8765-49e1-88ca-c153ad0276d0")]
    pub the_9685_b9_a9_876549_e1_88_ca_c153_ad0276_d0: Option<i64>,

    #[serde(rename = "979aee4a-6d80-4863-bf1c-ee1a78e06024")]
    pub the_979_aee4_a_6_d80_4863_bf1_c_ee1_a78_e06024: Option<i64>,

    #[serde(rename = "99edf2f4-f47d-46ee-998a-4cb4200236f7")]
    pub the_99_edf2_f4_f47_d_46_ee_998_a_4_cb4200236_f7: Option<i64>,

    #[serde(rename = "9a2f6bb9-c72c-437c-a3c4-e076dc5d10d4")]
    pub the_9_a2_f6_bb9_c72_c_437_c_a3_c4_e076_dc5_d10_d4: Option<i64>,

    #[serde(rename = "9da5c6b8-ccad-4bb5-b6b8-dc1d6b8ca6ed")]
    pub the_9_da5_c6_b8_ccad_4_bb5_b6_b8_dc1_d6_b8_ca6_ed: Option<i64>,

    #[serde(rename = "9debc64f-74b7-4ae1-a4d6-fce0144b6ea5")]
    pub the_9_debc64_f_74_b7_4_ae1_a4_d6_fce0144_b6_ea5: Option<i64>,

    #[serde(rename = "a01f0ade-0186-464d-8c68-e19a29cb66f0")]
    pub a01_f0_ade_0186464_d_8_c68_e19_a29_cb66_f0: Option<i64>,

    #[serde(rename = "a37f9158-7f82-46bc-908c-c9e2dda7c33b")]
    pub a37_f9158_7_f82_46_bc_908_c_c9_e2_dda7_c33_b: Option<i64>,

    #[serde(rename = "a554e084-b483-42da-89fb-39cd49ad7df6")]
    pub a554_e084_b483_42_da_89_fb_39_cd49_ad7_df6: Option<i64>,

    #[serde(rename = "a922603d-30c6-48d1-a83b-ae9be96675b6")]
    pub a922603_d_30_c6_48_d1_a83_b_ae9_be96675_b6: Option<i64>,

    #[serde(rename = "a94de6ef-5fc9-4470-89a0-557072fe4daf")]
    pub a94_de6_ef_5_fc9_447089_a0_557072_fe4_daf: Option<i64>,

    #[serde(rename = "adc5b394-8f76-416d-9ce9-813706877b84")]
    pub adc5_b394_8_f76_416_d_9_ce9_813706877_b84: Option<i64>,

    #[serde(rename = "ae0661b9-af66-4d4b-acc7-041e5cccb4bb")]
    pub ae0661_b9_af66_4_d4_b_acc7_041_e5_cccb4_bb: Option<i64>,

    #[serde(rename = "b024e975-1c4a-4575-8936-a3754a08806a")]
    pub b024_e975_1_c4_a_45758936_a3754_a08806_a: Option<i64>,

    #[serde(rename = "b069fdc6-2204-423a-932c-09037adcd845")]
    pub b069_fdc6_2204423_a_932_c_09037_adcd845: Option<i64>,

    #[serde(rename = "b1a50aa9-c515-46e8-8db9-d5378840362c")]
    pub b1_a50_aa9_c515_46_e8_8_db9_d5378840362_c: Option<i64>,

    #[serde(rename = "b320131f-da0d-43e1-9b98-f936a0ee417a")]
    pub b320131_f_da0_d_43_e1_9_b98_f936_a0_ee417_a: Option<i64>,

    #[serde(rename = "b35926d4-22a3-4419-8fab-686c41687055")]
    pub b35926_d4_22_a3_44198_fab_686_c41687055: Option<i64>,

    #[serde(rename = "b47df036-3aa4-4b98-8e9e-fe1d3ff1894b")]
    pub b47_df036_3_aa4_4_b98_8_e9_e_fe1_d3_ff1894_b: Option<i64>,

    #[serde(rename = "b5b4fb6b-08d8-401a-85d5-f08afa84af63")]
    pub b5_b4_fb6_b_08_d8_401_a_85_d5_f08_afa84_af63: Option<i64>,

    #[serde(rename = "b63be8c2-576a-4d6e-8daf-814f8bcea96f")]
    pub b63_be8_c2_576_a_4_d6_e_8_daf_814_f8_bcea96_f: Option<i64>,

    #[serde(rename = "b6b5df8f-5602-4883-b47d-07e77ed9d5af")]
    pub b6_b5_df8_f_56024883_b47_d_07_e77_ed9_d5_af: Option<i64>,

    #[serde(rename = "b72f3061-f573-40d7-832a-5ad475bd7909")]
    pub b72_f3061_f573_40_d7_832_a_5_ad475_bd7909: Option<i64>,

    #[serde(rename = "b7df2ea6-f4e8-4e6b-8c98-f730701f3717")]
    pub b7_df2_ea6_f4_e8_4_e6_b_8_c98_f730701_f3717: Option<i64>,

    #[serde(rename = "b7f9cc0c-6a6c-4bed-adbb-2d2d2dfbe810")]
    pub b7_f9_cc0_c_6_a6_c_4_bed_adbb_2_d2_d2_dfbe810: Option<i64>,

    #[serde(rename = "ba6d5599-1242-41ed-be64-90de7b1c255f")]
    pub ba6_d5599_124241_ed_be64_90_de7_b1_c255_f: Option<i64>,

    #[serde(rename = "bb4a9de5-c924-4923-a0cb-9d1445f1ee5d")]
    pub bb4_a9_de5_c924_4923_a0_cb_9_d1445_f1_ee5_d: Option<i64>,

    #[serde(rename = "bebf13f9-82d1-4133-9b14-4a96de029ccf")]
    pub bebf13_f9_82_d1_41339_b14_4_a96_de029_ccf: Option<i64>,

    #[serde(rename = "bfd38797-8404-4b38-8b82-341da28b1f83")]
    pub bfd38797_84044_b38_8_b82_341_da28_b1_f83: Option<i64>,

    #[serde(rename = "c0dc2c80-463e-49f7-9e00-c62473d677c8")]
    pub c0_dc2_c80_463_e_49_f7_9_e00_c62473_d677_c8: Option<i64>,

    #[serde(rename = "c19bb50b-9a22-4dd2-8200-bce639b1b239")]
    pub c19_bb50_b_9_a22_4_dd2_8200_bce639_b1_b239: Option<i64>,

    #[serde(rename = "c73b705c-40ad-4633-a6ed-d357ee2e2bcf")]
    pub c73_b705_c_40_ad_4633_a6_ed_d357_ee2_e2_bcf: Option<i64>,

    #[serde(rename = "c794d5aa-6104-420e-ae6f-3b2c270253fd")]
    pub c794_d5_aa_6104420_e_ae6_f_3_b2_c270253_fd: Option<i64>,

    #[serde(rename = "ca117809-cda1-4ae0-b607-53079fb5b133")]
    pub ca117809_cda1_4_ae0_b607_53079_fb5_b133: Option<i64>,

    #[serde(rename = "ca3f1c8c-c025-4d8e-8eef-5be6accbeb16")]
    pub ca3_f1_c8_c_c025_4_d8_e_8_eef_5_be6_accbeb16: Option<i64>,

    #[serde(rename = "cade1731-39a8-43f3-be8e-d2302711fe8b")]
    pub cade1731_39_a8_43_f3_be8_e_d2302711_fe8_b: Option<i64>,

    #[serde(rename = "cbd44c06-231a-4d1a-bb7d-4170b06e566a")]
    pub cbd44_c06_231_a_4_d1_a_bb7_d_4170_b06_e566_a: Option<i64>,

    #[serde(rename = "cc9de838-4431-4cc7-9c3e-15b15b2142b0")]
    pub cc9_de838_44314_cc7_9_c3_e_15_b15_b2142_b0: Option<i64>,

    #[serde(rename = "cd29d13d-99d4-414b-8faa-f0819b2de526")]
    pub cd29_d13_d_99_d4_414_b_8_faa_f0819_b2_de526: Option<i64>,

    #[serde(rename = "d0762a7e-004b-48a9-a832-a993982b305b")]
    pub d0762_a7_e_004_b_48_a9_a832_a993982_b305_b: Option<i64>,

    #[serde(rename = "d2874e7f-8e88-442a-a176-e256df68a49b")]
    pub d2874_e7_f_8_e88_442_a_a176_e256_df68_a49_b: Option<i64>,

    #[serde(rename = "d2949bd0-6a28-4e0d-aa07-cecc437cbd99")]
    pub d2949_bd0_6_a28_4_e0_d_aa07_cecc437_cbd99: Option<i64>,

    #[serde(rename = "d2c33336-b5a9-4ce1-86bb-f376ec66efbd")]
    pub d2_c33336_b5_a9_4_ce1_86_bb_f376_ec66_efbd: Option<i64>,

    #[serde(rename = "d6a352fc-b675-40a0-864d-f4fd50aaeea0")]
    pub d6_a352_fc_b675_40_a0_864_d_f4_fd50_aaeea0: Option<i64>,

    #[serde(rename = "d82a1a80-dff3-4767-bab6-484b2eb7aee1")]
    pub d82_a1_a80_dff3_4767_bab6_484_b2_eb7_aee1: Option<i64>,

    #[serde(rename = "d9f89a8a-c563-493e-9d64-78e4f9a55d4a")]
    pub d9_f89_a8_a_c563_493_e_9_d64_78_e4_f9_a55_d4_a: Option<i64>,

    #[serde(rename = "e11df0cc-3a95-4159-9a84-fecbbf23ae05")]
    pub e11_df0_cc_3_a95_41599_a84_fecbbf23_ae05: Option<i64>,

    #[serde(rename = "e12313fe-c0c9-49de-9d11-8b7408aa92ce")]
    pub e12313_fe_c0_c9_49_de_9_d11_8_b7408_aa92_ce: Option<i64>,

    #[serde(rename = "e4f7549c-17af-4e35-b89b-f0fae855a31b")]
    pub e4_f7549_c_17_af_4_e35_b89_b_f0_fae855_a31_b: Option<i64>,

    #[serde(rename = "ea3c8019-b6b6-4830-b952-7e9c2ce707bd")]
    pub ea3_c8019_b6_b6_4830_b952_7_e9_c2_ce707_bd: Option<i64>,

    #[serde(rename = "eb67ae5e-c4bf-46ca-bbbc-425cd34182ff")]
    pub eb67_ae5_e_c4_bf_46_ca_bbbc_425_cd34182_ff: Option<i64>,

    #[serde(rename = "ee722cbd-812f-4525-81d7-dfa89fb867a4")]
    pub ee722_cbd_812_f_452581_d7_dfa89_fb867_a4: Option<i64>,

    #[serde(rename = "effdbd8d-a54f-4049-a3c8-b5f944e5278b")]
    pub effdbd8_d_a54_f_4049_a3_c8_b5_f944_e5278_b: Option<i64>,

    #[serde(rename = "f02aeae2-5e6a-4098-9842-02d2273f25c7")]
    pub f02_aeae2_5_e6_a_4098984202_d2273_f25_c7: Option<i64>,

    #[serde(rename = "f0ec8435-0427-4ffd-ad0c-a67f60a75e0e")]
    pub f0_ec8435_04274_ffd_ad0_c_a67_f60_a75_e0_e: Option<i64>,

    #[serde(rename = "f3490435-a42f-42a8-ab89-d59e8dc8d599")]
    pub f3490435_a42_f_42_a8_ab89_d59_e8_dc8_d599: Option<i64>,

    #[serde(rename = "f8d99dc7-ae37-4f35-b08c-543864a347f2")]
    pub f8_d99_dc7_ae37_4_f35_b08_c_543864_a347_f2: Option<i64>,

    #[serde(rename = "f9045b82-5570-43d4-856b-bed5095515c6")]
    pub f9045_b82_557043_d4_856_b_bed5095515_c6: Option<i64>,

    #[serde(rename = "fab9420f-0730-4054-bd17-355113f204c2")]
    pub fab9420_f_07304054_bd17_355113_f204_c2: Option<i64>,

    #[serde(rename = "fca16c92-5f03-45b9-abbe-760866878ffe")]
    pub fca16_c92_5_f03_45_b9_abbe_760866878_ffe: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Runs {
    #[serde(rename = "045b4b38-fb11-4fa6-8dc0-f75997eacd28")]
    pub the_045_b4_b38_fb11_4_fa6_8_dc0_f75997_eacd28: Option<i64>,

    #[serde(rename = "0706f3cf-d6c4-4bd0-ac8c-de3d75ffa77e")]
    pub the_0706_f3_cf_d6_c4_4_bd0_ac8_c_de3_d75_ffa77_e: Option<i64>,

    #[serde(rename = "074b5e4a-84f8-428d-884e-4592a77ee061")]
    pub the_074_b5_e4_a_84_f8_428_d_884_e_4592_a77_ee061: Option<i64>,

    #[serde(rename = "09a77dd0-13c6-4c18-870a-63cd005ddff6")]
    pub the_09_a77_dd0_13_c6_4_c18_870_a_63_cd005_ddff6: Option<i64>,

    #[serde(rename = "0a449b4d-504b-448c-9516-6027fd6d216e")]
    pub the_0_a449_b4_d_504_b_448_c_95166027_fd6_d216_e: Option<i64>,

    #[serde(rename = "0b672007-ebfb-476d-8fdb-fb66bad78df2")]
    pub the_0_b672007_ebfb_476_d_8_fdb_fb66_bad78_df2: Option<i64>,

    #[serde(rename = "0eae657b-3592-43fb-93e1-b878680d2b53")]
    pub the_0_eae657_b_359243_fb_93_e1_b878680_d2_b53: Option<i64>,

    #[serde(rename = "105bc3ff-1320-4e37-8ef0-8d595cb95dd0")]
    pub the_105_bc3_ff_13204_e37_8_ef0_8_d595_cb95_dd0: Option<f64>,

    #[serde(rename = "110a62be-bc8a-4f0c-8046-0eb580b1af1c")]
    pub the_110_a62_be_bc8_a_4_f0_c_80460_eb580_b1_af1_c: Option<i64>,

    #[serde(rename = "11b92425-aa46-4691-b42f-baa2b6ddb541")]
    pub the_11_b92425_aa46_4691_b42_f_baa2_b6_ddb541: Option<i64>,

    #[serde(rename = "16be150e-372e-453e-b6ff-597aa42ca5ee")]
    pub the_16_be150_e_372_e_453_e_b6_ff_597_aa42_ca5_ee: Option<i64>,

    #[serde(rename = "16d1fd9b-c62b-4bed-b68a-b3a2d6e21524")]
    pub the_16_d1_fd9_b_c62_b_4_bed_b68_a_b3_a2_d6_e21524: Option<i64>,

    #[serde(rename = "19f81c84-9a94-49fa-9c23-2e1355126250")]
    pub the_19_f81_c84_9_a94_49_fa_9_c23_2_e1355126250: Option<i64>,

    #[serde(rename = "1a51664e-efec-45fa-b0ba-06d04c344628")]
    pub the_1_a51664_e_efec_45_fa_b0_ba_06_d04_c344628: Option<i64>,

    #[serde(rename = "1e04e5cc-80a6-41c0-af0d-7292817eed79")]
    pub the_1_e04_e5_cc_80_a6_41_c0_af0_d_7292817_eed79: Option<i64>,

    #[serde(rename = "22d8a1e9-e679-4bde-ae8a-318cb591d1c8")]
    pub the_22_d8_a1_e9_e679_4_bde_ae8_a_318_cb591_d1_c8: Option<i64>,

    #[serde(rename = "23a2cea4-5df7-4ed0-bb2c-b8c297518ada")]
    pub the_23_a2_cea4_5_df7_4_ed0_bb2_c_b8_c297518_ada: Option<i64>,

    #[serde(rename = "23e4cbc1-e9cd-47fa-a35b-bfa06f726cb7")]
    pub the_23_e4_cbc1_e9_cd_47_fa_a35_b_bfa06_f726_cb7: Option<f64>,

    #[serde(rename = "258f6389-aac1-43d2-b30a-4b4dde90d5eb")]
    pub the_258_f6389_aac1_43_d2_b30_a_4_b4_dde90_d5_eb: Option<i64>,

    #[serde(rename = "2957236a-6077-4012-a445-8c5be111afd0")]
    pub the_2957236_a_60774012_a445_8_c5_be111_afd0: Option<i64>,

    #[serde(rename = "2d02b60b-d858-4b8a-a835-c1e8fe1b8fe0")]
    pub the_2_d02_b60_b_d858_4_b8_a_a835_c1_e8_fe1_b8_fe0: Option<i64>,

    #[serde(rename = "2d07beca-bdb1-4ecb-bcfe-5913e2b406f5")]
    pub the_2_d07_beca_bdb1_4_ecb_bcfe_5913_e2_b406_f5: Option<i64>,

    #[serde(rename = "2dc7a1fa-3ae6-47ed-8c92-5d80167959f5")]
    pub the_2_dc7_a1_fa_3_ae6_47_ed_8_c92_5_d80167959_f5: Option<i64>,

    #[serde(rename = "2de5c38c-3d72-4d97-af0f-15e98eba2225")]
    pub the_2_de5_c38_c_3_d72_4_d97_af0_f_15_e98_eba2225: Option<i64>,

    #[serde(rename = "2e22beba-8e36-42ba-a8bf-975683c52b5f")]
    pub the_2_e22_beba_8_e36_42_ba_a8_bf_975683_c52_b5_f: Option<f64>,

    #[serde(rename = "2ec5927e-9905-408c-a04c-65a8879f846a")]
    pub the_2_ec5927_e_9905408_c_a04_c_65_a8879_f846_a: Option<i64>,

    #[serde(rename = "30c9bcd2-cc5a-421d-97d0-d39fefad053a")]
    pub the_30_c9_bcd2_cc5_a_421_d_97_d0_d39_fefad053_a: Option<i64>,

    #[serde(rename = "34a2e6ca-08fd-468e-894b-c707d6ce460a")]
    pub the_34_a2_e6_ca_08_fd_468_e_894_b_c707_d6_ce460_a: Option<i64>,

    #[serde(rename = "36569151-a2fb-43c1-9df7-2df512424c82")]
    pub the_36569151_a2_fb_43_c1_9_df7_2_df512424_c82: Option<f64>,

    #[serde(rename = "365b4517-4b0a-45da-aaa6-161dd77de99a")]
    pub the_365_b4517_4_b0_a_45_da_aaa6_161_dd77_de99_a: Option<i64>,

    #[serde(rename = "36f4efea-9d27-4457-a7b4-4b45ad2e23a3")]
    pub the_36_f4_efea_9_d27_4457_a7_b4_4_b45_ad2_e23_a3: Option<i64>,

    #[serde(rename = "378e3344-1a1a-4332-80cc-3da45954a4f4")]
    pub the_378_e3344_1_a1_a_433280_cc_3_da45954_a4_f4: Option<i64>,

    #[serde(rename = "3a094991-4cbc-4786-b74c-688876d243f4")]
    pub the_3_a094991_4_cbc_4786_b74_c_688876_d243_f4: Option<i64>,

    #[serde(rename = "3a4412d6-5404-4801-bf06-81cb7884fae4")]
    pub the_3_a4412_d6_54044801_bf06_81_cb7884_fae4: Option<i64>,

    #[serde(rename = "3aba36e6-9dd7-417f-9d3a-69d778439020")]
    pub the_3_aba36_e6_9_dd7_417_f_9_d3_a_69_d778439020: Option<i64>,

    #[serde(rename = "3b1c5a25-ed79-4ce2-87d4-0c1cf3ff342e")]
    pub the_3_b1_c5_a25_ed79_4_ce2_87_d4_0_c1_cf3_ff342_e: Option<i64>,

    #[serde(rename = "3cea1405-3a5f-432c-96c3-a85dc7f163ee")]
    pub the_3_cea1405_3_a5_f_432_c_96_c3_a85_dc7_f163_ee: Option<i64>,

    #[serde(rename = "3d858bda-dcef-4d05-928e-6557d3123f17")]
    pub the_3_d858_bda_dcef_4_d05_928_e_6557_d3123_f17: Option<i64>,

    #[serde(rename = "3f8bbb15-61c0-4e3f-8e4a-907a5fb1565e")]
    pub the_3_f8_bbb15_61_c0_4_e3_f_8_e4_a_907_a5_fb1565_e: Option<f64>,

    #[serde(rename = "444f2846-8927-4271-b2ca-6bf8b5b94610")]
    pub the_444_f2846_89274271_b2_ca_6_bf8_b5_b94610: Option<i64>,

    #[serde(rename = "44d9dc46-7e81-4e21-acff-c0f5dd399ae3")]
    pub the_44_d9_dc46_7_e81_4_e21_acff_c0_f5_dd399_ae3: Option<i64>,

    #[serde(rename = "46358869-dce9-4a01-bfba-ac24fc56f57e")]
    pub the_46358869_dce9_4_a01_bfba_ac24_fc56_f57_e: Option<f64>,

    #[serde(rename = "4b1004bc-345e-4084-8d18-b46315624864")]
    pub the_4_b1004_bc_345_e_40848_d18_b46315624864: Option<i64>,

    #[serde(rename = "4c192065-65d8-4010-8145-395f82d24ddf")]
    pub the_4_c192065_65_d8_40108145395_f82_d24_ddf: Option<i64>,

    #[serde(rename = "4cd14d96-f817-41a3-af6c-2d3ed0dd20b7")]
    pub the_4_cd14_d96_f817_41_a3_af6_c_2_d3_ed0_dd20_b7: Option<i64>,

    #[serde(rename = "4cf31f0e-fb42-4933-8fdb-fde58d109ced")]
    pub the_4_cf31_f0_e_fb42_49338_fdb_fde58_d109_ced: Option<i64>,

    #[serde(rename = "4d5f27b5-8924-498f-aa4c-7f5967c0c7c6")]
    pub the_4_d5_f27_b5_8924498_f_aa4_c_7_f5967_c0_c7_c6: Option<i64>,

    #[serde(rename = "505ae98b-7d85-4f51-99ef-60ccd7365d97")]
    pub the_505_ae98_b_7_d85_4_f51_99_ef_60_ccd7365_d97: Option<i64>,

    #[serde(rename = "5371833b-a620-4952-b2cb-a15eed8ad183")]
    pub the_5371833_b_a620_4952_b2_cb_a15_eed8_ad183: Option<i64>,

    #[serde(rename = "54d0d0f2-16e0-42a0-9fff-79cfa7c4a157")]
    pub the_54_d0_d0_f2_16_e0_42_a0_9_fff_79_cfa7_c4_a157: Option<i64>,

    #[serde(rename = "55c9fee3-79c8-4467-8dfb-ff1e340aae8c")]
    pub the_55_c9_fee3_79_c8_44678_dfb_ff1_e340_aae8_c: Option<i64>,

    #[serde(rename = "5663778c-c8fb-4408-908a-31dc1f6c55cc")]
    pub the_5663778_c_c8_fb_4408908_a_31_dc1_f6_c55_cc: Option<i64>,

    #[serde(rename = "5666fce7-3b39-4ade-9220-71244d9be5d8")]
    pub the_5666_fce7_3_b39_4_ade_922071244_d9_be5_d8: Option<i64>,

    #[serde(rename = "57d3f614-f8d3-4dfd-b486-075f823fdb0b")]
    pub the_57_d3_f614_f8_d3_4_dfd_b486_075_f823_fdb0_b: Option<i64>,

    #[serde(rename = "57ec08cc-0411-4643-b304-0e80dbc15ac7")]
    pub the_57_ec08_cc_04114643_b304_0_e80_dbc15_ac7: Option<f64>,

    #[serde(rename = "5818fb9b-f191-462e-9085-6fe311aaaf70")]
    pub the_5818_fb9_b_f191_462_e_90856_fe311_aaaf70: Option<i64>,

    #[serde(rename = "5d3bd8ab-cc9a-4aa5-bbcd-fa0f96566c64")]
    pub the_5_d3_bd8_ab_cc9_a_4_aa5_bbcd_fa0_f96566_c64: Option<i64>,

    #[serde(rename = "628bb28b-306e-4ff7-ad02-05524bcf246a")]
    pub the_628_bb28_b_306_e_4_ff7_ad02_05524_bcf246_a: Option<i64>,

    #[serde(rename = "635c332d-6ea9-4766-b391-ae4c3435f677")]
    pub the_635_c332_d_6_ea9_4766_b391_ae4_c3435_f677: Option<i64>,

    #[serde(rename = "6526d5df-6a9c-48e1-ba50-12dec0d8b22f")]
    pub the_6526_d5_df_6_a9_c_48_e1_ba50_12_dec0_d8_b22_f: Option<i64>,

    #[serde(rename = "6e655fc7-5190-4e55-99a0-89683d443cfc")]
    pub the_6_e655_fc7_51904_e55_99_a0_89683_d443_cfc: Option<i64>,

    #[serde(rename = "6f9ff34d-825f-477b-8600-1cec4febaecf")]
    pub the_6_f9_ff34_d_825_f_477_b_86001_cec4_febaecf: Option<i64>,

    #[serde(rename = "70167f08-5e85-44d7-b047-2961201c1615")]
    pub the_70167_f08_5_e85_44_d7_b047_2961201_c1615: Option<i64>,

    #[serde(rename = "71b157bc-8a50-4c05-a785-034f660e493f")]
    pub the_71_b157_bc_8_a50_4_c05_a785_034_f660_e493_f: Option<i64>,

    #[serde(rename = "747b8e4a-7e50-4638-a973-ea7950a3e739")]
    pub the_747_b8_e4_a_7_e50_4638_a973_ea7950_a3_e739: Option<f64>,

    #[serde(rename = "74aea6b6-34f9-48f4-b298-7345e1f9f7cb")]
    pub the_74_aea6_b6_34_f9_48_f4_b298_7345_e1_f9_f7_cb: Option<i64>,

    #[serde(rename = "75667373-b350-499b-b86e-5518b6f9f6ab")]
    pub the_75667373_b350_499_b_b86_e_5518_b6_f9_f6_ab: Option<i64>,

    #[serde(rename = "76d3489f-c7c4-4cb9-9c58-b1e1bab062d1")]
    pub the_76_d3489_f_c7_c4_4_cb9_9_c58_b1_e1_bab062_d1: Option<i64>,

    #[serde(rename = "780054a7-74ee-44fd-ab5f-6dd4637c5ef1")]
    pub the_780054_a7_74_ee_44_fd_ab5_f_6_dd4637_c5_ef1: Option<i64>,

    #[serde(rename = "79347640-8d5d-4e41-819d-2b0c86f20b76")]
    pub the_793476408_d5_d_4_e41_819_d_2_b0_c86_f20_b76: Option<i64>,

    #[serde(rename = "7966eb04-efcc-499b-8f03-d13916330531")]
    pub the_7966_eb04_efcc_499_b_8_f03_d13916330531: Option<f64>,

    #[serde(rename = "7ce9e0b0-9639-45f1-8db6-32c30ca0012d")]
    pub the_7_ce9_e0_b0_963945_f1_8_db6_32_c30_ca0012_d: Option<i64>,

    #[serde(rename = "7dc37924-0bb8-4e40-a826-c497d51e447c")]
    pub the_7_dc37924_0_bb8_4_e40_a826_c497_d51_e447_c: Option<i64>,

    #[serde(rename = "86435ed2-d205-4c37-be22-89683b9a7a62")]
    pub the_86435_ed2_d205_4_c37_be22_89683_b9_a7_a62: Option<i64>,

    #[serde(rename = "86f4485a-a6db-470b-82f5-e95e6b353537")]
    pub the_86_f4485_a_a6_db_470_b_82_f5_e95_e6_b353537: Option<i64>,

    #[serde(rename = "8756d8e1-bd9d-4116-8fd0-5ea06c2e80c3")]
    pub the_8756_d8_e1_bd9_d_41168_fd0_5_ea06_c2_e80_c3: Option<i64>,

    #[serde(rename = "878c1bf6-0d21-4659-bfee-916c8314d69c")]
    pub the_878_c1_bf6_0_d21_4659_bfee_916_c8314_d69_c: Option<f64>,

    #[serde(rename = "88151292-6c12-4fb8-b2d6-3e64821293b3")]
    pub the_881512926_c12_4_fb8_b2_d6_3_e64821293_b3: Option<i64>,

    #[serde(rename = "89796ffb-843a-4163-8dec-1bef229c68cb")]
    pub the_89796_ffb_843_a_41638_dec_1_bef229_c68_cb: Option<i64>,

    #[serde(rename = "8981c839-cbcf-47e3-a74e-8731dcff24fe")]
    pub the_8981_c839_cbcf_47_e3_a74_e_8731_dcff24_fe: Option<i64>,

    #[serde(rename = "8aaf714b-d42a-40b0-9165-384befc66d55")]
    pub the_8_aaf714_b_d42_a_40_b0_9165384_befc66_d55: Option<i64>,

    #[serde(rename = "8b38afb3-2e20-4e73-bb00-22bab14e3cda")]
    pub the_8_b38_afb3_2_e20_4_e73_bb00_22_bab14_e3_cda: Option<i64>,

    #[serde(rename = "8d7ba290-5f87-403c-81e3-cf5a2b6a6082")]
    pub the_8_d7_ba290_5_f87_403_c_81_e3_cf5_a2_b6_a6082: Option<i64>,

    #[serde(rename = "8d87c468-699a-47a8-b40d-cfb73a5660ad")]
    pub the_8_d87_c468_699_a_47_a8_b40_d_cfb73_a5660_ad: Option<f64>,

    #[serde(rename = "910974d7-cbfd-4d2d-8733-7e85759932da")]
    pub the_910974_d7_cbfd_4_d2_d_87337_e85759932_da: Option<i64>,

    #[serde(rename = "93e71a0e-80fc-46b7-beaf-d204c425fe03")]
    pub the_93_e71_a0_e_80_fc_46_b7_beaf_d204_c425_fe03: Option<i64>,

    #[serde(rename = "93f91157-f628-4c9a-a392-d2b1dbd79ac5")]
    pub the_93_f91157_f628_4_c9_a_a392_d2_b1_dbd79_ac5: Option<i64>,

    #[serde(rename = "9494152b-99f6-4adb-9573-f9e084bc813f")]
    pub the_9494152_b_99_f6_4_adb_9573_f9_e084_bc813_f: Option<i64>,

    #[serde(rename = "9657f04e-1c51-4951-88de-d376bb57f5bd")]
    pub the_9657_f04_e_1_c51_495188_de_d376_bb57_f5_bd: Option<i64>,

    #[serde(rename = "9685b9a9-8765-49e1-88ca-c153ad0276d0")]
    pub the_9685_b9_a9_876549_e1_88_ca_c153_ad0276_d0: Option<i64>,

    #[serde(rename = "979aee4a-6d80-4863-bf1c-ee1a78e06024")]
    pub the_979_aee4_a_6_d80_4863_bf1_c_ee1_a78_e06024: Option<f64>,

    #[serde(rename = "99edf2f4-f47d-46ee-998a-4cb4200236f7")]
    pub the_99_edf2_f4_f47_d_46_ee_998_a_4_cb4200236_f7: Option<i64>,

    #[serde(rename = "9a2f6bb9-c72c-437c-a3c4-e076dc5d10d4")]
    pub the_9_a2_f6_bb9_c72_c_437_c_a3_c4_e076_dc5_d10_d4: Option<i64>,

    #[serde(rename = "9da5c6b8-ccad-4bb5-b6b8-dc1d6b8ca6ed")]
    pub the_9_da5_c6_b8_ccad_4_bb5_b6_b8_dc1_d6_b8_ca6_ed: Option<i64>,

    #[serde(rename = "9debc64f-74b7-4ae1-a4d6-fce0144b6ea5")]
    pub the_9_debc64_f_74_b7_4_ae1_a4_d6_fce0144_b6_ea5: Option<f64>,

    #[serde(rename = "a01f0ade-0186-464d-8c68-e19a29cb66f0")]
    pub a01_f0_ade_0186464_d_8_c68_e19_a29_cb66_f0: Option<i64>,

    #[serde(rename = "a37f9158-7f82-46bc-908c-c9e2dda7c33b")]
    pub a37_f9158_7_f82_46_bc_908_c_c9_e2_dda7_c33_b: Option<f64>,

    #[serde(rename = "a554e084-b483-42da-89fb-39cd49ad7df6")]
    pub a554_e084_b483_42_da_89_fb_39_cd49_ad7_df6: Option<i64>,

    #[serde(rename = "a922603d-30c6-48d1-a83b-ae9be96675b6")]
    pub a922603_d_30_c6_48_d1_a83_b_ae9_be96675_b6: Option<i64>,

    #[serde(rename = "a94de6ef-5fc9-4470-89a0-557072fe4daf")]
    pub a94_de6_ef_5_fc9_447089_a0_557072_fe4_daf: Option<i64>,

    #[serde(rename = "adc5b394-8f76-416d-9ce9-813706877b84")]
    pub adc5_b394_8_f76_416_d_9_ce9_813706877_b84: Option<f64>,

    #[serde(rename = "ae0661b9-af66-4d4b-acc7-041e5cccb4bb")]
    pub ae0661_b9_af66_4_d4_b_acc7_041_e5_cccb4_bb: Option<i64>,

    #[serde(rename = "b024e975-1c4a-4575-8936-a3754a08806a")]
    pub b024_e975_1_c4_a_45758936_a3754_a08806_a: Option<f64>,

    #[serde(rename = "b069fdc6-2204-423a-932c-09037adcd845")]
    pub b069_fdc6_2204423_a_932_c_09037_adcd845: Option<i64>,

    #[serde(rename = "b1a50aa9-c515-46e8-8db9-d5378840362c")]
    pub b1_a50_aa9_c515_46_e8_8_db9_d5378840362_c: Option<i64>,

    #[serde(rename = "b320131f-da0d-43e1-9b98-f936a0ee417a")]
    pub b320131_f_da0_d_43_e1_9_b98_f936_a0_ee417_a: Option<i64>,

    #[serde(rename = "b35926d4-22a3-4419-8fab-686c41687055")]
    pub b35926_d4_22_a3_44198_fab_686_c41687055: Option<i64>,

    #[serde(rename = "b47df036-3aa4-4b98-8e9e-fe1d3ff1894b")]
    pub b47_df036_3_aa4_4_b98_8_e9_e_fe1_d3_ff1894_b: Option<f64>,

    #[serde(rename = "b5b4fb6b-08d8-401a-85d5-f08afa84af63")]
    pub b5_b4_fb6_b_08_d8_401_a_85_d5_f08_afa84_af63: Option<i64>,

    #[serde(rename = "b63be8c2-576a-4d6e-8daf-814f8bcea96f")]
    pub b63_be8_c2_576_a_4_d6_e_8_daf_814_f8_bcea96_f: Option<f64>,

    #[serde(rename = "b6b5df8f-5602-4883-b47d-07e77ed9d5af")]
    pub b6_b5_df8_f_56024883_b47_d_07_e77_ed9_d5_af: Option<i64>,

    #[serde(rename = "b72f3061-f573-40d7-832a-5ad475bd7909")]
    pub b72_f3061_f573_40_d7_832_a_5_ad475_bd7909: Option<f64>,

    #[serde(rename = "b7df2ea6-f4e8-4e6b-8c98-f730701f3717")]
    pub b7_df2_ea6_f4_e8_4_e6_b_8_c98_f730701_f3717: Option<i64>,

    #[serde(rename = "b7f9cc0c-6a6c-4bed-adbb-2d2d2dfbe810")]
    pub b7_f9_cc0_c_6_a6_c_4_bed_adbb_2_d2_d2_dfbe810: Option<i64>,

    #[serde(rename = "ba6d5599-1242-41ed-be64-90de7b1c255f")]
    pub ba6_d5599_124241_ed_be64_90_de7_b1_c255_f: Option<i64>,

    #[serde(rename = "bb4a9de5-c924-4923-a0cb-9d1445f1ee5d")]
    pub bb4_a9_de5_c924_4923_a0_cb_9_d1445_f1_ee5_d: Option<f64>,

    #[serde(rename = "bebf13f9-82d1-4133-9b14-4a96de029ccf")]
    pub bebf13_f9_82_d1_41339_b14_4_a96_de029_ccf: Option<i64>,

    #[serde(rename = "bfd38797-8404-4b38-8b82-341da28b1f83")]
    pub bfd38797_84044_b38_8_b82_341_da28_b1_f83: Option<f64>,

    #[serde(rename = "c0dc2c80-463e-49f7-9e00-c62473d677c8")]
    pub c0_dc2_c80_463_e_49_f7_9_e00_c62473_d677_c8: Option<i64>,

    #[serde(rename = "c19bb50b-9a22-4dd2-8200-bce639b1b239")]
    pub c19_bb50_b_9_a22_4_dd2_8200_bce639_b1_b239: Option<i64>,

    #[serde(rename = "c73b705c-40ad-4633-a6ed-d357ee2e2bcf")]
    pub c73_b705_c_40_ad_4633_a6_ed_d357_ee2_e2_bcf: Option<f64>,

    #[serde(rename = "c794d5aa-6104-420e-ae6f-3b2c270253fd")]
    pub c794_d5_aa_6104420_e_ae6_f_3_b2_c270253_fd: Option<i64>,

    #[serde(rename = "ca117809-cda1-4ae0-b607-53079fb5b133")]
    pub ca117809_cda1_4_ae0_b607_53079_fb5_b133: Option<i64>,

    #[serde(rename = "ca3f1c8c-c025-4d8e-8eef-5be6accbeb16")]
    pub ca3_f1_c8_c_c025_4_d8_e_8_eef_5_be6_accbeb16: Option<f64>,

    #[serde(rename = "cade1731-39a8-43f3-be8e-d2302711fe8b")]
    pub cade1731_39_a8_43_f3_be8_e_d2302711_fe8_b: Option<i64>,

    #[serde(rename = "cbd44c06-231a-4d1a-bb7d-4170b06e566a")]
    pub cbd44_c06_231_a_4_d1_a_bb7_d_4170_b06_e566_a: Option<i64>,

    #[serde(rename = "cc9de838-4431-4cc7-9c3e-15b15b2142b0")]
    pub cc9_de838_44314_cc7_9_c3_e_15_b15_b2142_b0: Option<i64>,

    #[serde(rename = "cd29d13d-99d4-414b-8faa-f0819b2de526")]
    pub cd29_d13_d_99_d4_414_b_8_faa_f0819_b2_de526: Option<i64>,

    #[serde(rename = "d0762a7e-004b-48a9-a832-a993982b305b")]
    pub d0762_a7_e_004_b_48_a9_a832_a993982_b305_b: Option<i64>,

    #[serde(rename = "d2874e7f-8e88-442a-a176-e256df68a49b")]
    pub d2874_e7_f_8_e88_442_a_a176_e256_df68_a49_b: Option<i64>,

    #[serde(rename = "d2949bd0-6a28-4e0d-aa07-cecc437cbd99")]
    pub d2949_bd0_6_a28_4_e0_d_aa07_cecc437_cbd99: Option<i64>,

    #[serde(rename = "d2c33336-b5a9-4ce1-86bb-f376ec66efbd")]
    pub d2_c33336_b5_a9_4_ce1_86_bb_f376_ec66_efbd: Option<i64>,

    #[serde(rename = "d6a352fc-b675-40a0-864d-f4fd50aaeea0")]
    pub d6_a352_fc_b675_40_a0_864_d_f4_fd50_aaeea0: Option<i64>,

    #[serde(rename = "d82a1a80-dff3-4767-bab6-484b2eb7aee1")]
    pub d82_a1_a80_dff3_4767_bab6_484_b2_eb7_aee1: Option<i64>,

    #[serde(rename = "d9f89a8a-c563-493e-9d64-78e4f9a55d4a")]
    pub d9_f89_a8_a_c563_493_e_9_d64_78_e4_f9_a55_d4_a: Option<f64>,

    #[serde(rename = "e11df0cc-3a95-4159-9a84-fecbbf23ae05")]
    pub e11_df0_cc_3_a95_41599_a84_fecbbf23_ae05: Option<i64>,

    #[serde(rename = "e12313fe-c0c9-49de-9d11-8b7408aa92ce")]
    pub e12313_fe_c0_c9_49_de_9_d11_8_b7408_aa92_ce: Option<i64>,

    #[serde(rename = "e4f7549c-17af-4e35-b89b-f0fae855a31b")]
    pub e4_f7549_c_17_af_4_e35_b89_b_f0_fae855_a31_b: Option<i64>,

    #[serde(rename = "ea3c8019-b6b6-4830-b952-7e9c2ce707bd")]
    pub ea3_c8019_b6_b6_4830_b952_7_e9_c2_ce707_bd: Option<i64>,

    #[serde(rename = "eb67ae5e-c4bf-46ca-bbbc-425cd34182ff")]
    pub eb67_ae5_e_c4_bf_46_ca_bbbc_425_cd34182_ff: Option<f64>,

    #[serde(rename = "ee722cbd-812f-4525-81d7-dfa89fb867a4")]
    pub ee722_cbd_812_f_452581_d7_dfa89_fb867_a4: Option<i64>,

    #[serde(rename = "effdbd8d-a54f-4049-a3c8-b5f944e5278b")]
    pub effdbd8_d_a54_f_4049_a3_c8_b5_f944_e5278_b: Option<i64>,

    #[serde(rename = "f02aeae2-5e6a-4098-9842-02d2273f25c7")]
    pub f02_aeae2_5_e6_a_4098984202_d2273_f25_c7: Option<f64>,

    #[serde(rename = "f0ec8435-0427-4ffd-ad0c-a67f60a75e0e")]
    pub f0_ec8435_04274_ffd_ad0_c_a67_f60_a75_e0_e: Option<i64>,

    #[serde(rename = "f3490435-a42f-42a8-ab89-d59e8dc8d599")]
    pub f3490435_a42_f_42_a8_ab89_d59_e8_dc8_d599: Option<i64>,

    #[serde(rename = "f8d99dc7-ae37-4f35-b08c-543864a347f2")]
    pub f8_d99_dc7_ae37_4_f35_b08_c_543864_a347_f2: Option<i64>,

    #[serde(rename = "f9045b82-5570-43d4-856b-bed5095515c6")]
    pub f9045_b82_557043_d4_856_b_bed5095515_c6: Option<i64>,

    #[serde(rename = "fab9420f-0730-4054-bd17-355113f204c2")]
    pub fab9420_f_07304054_bd17_355113_f204_c2: Option<i64>,

    #[serde(rename = "fca16c92-5f03-45b9-abbe-760866878ffe")]
    pub fca16_c92_5_f03_45_b9_abbe_760866878_ffe: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Wins {
    #[serde(rename = "045b4b38-fb11-4fa6-8dc0-f75997eacd28")]
    pub the_045_b4_b38_fb11_4_fa6_8_dc0_f75997_eacd28: Option<i64>,

    #[serde(rename = "0706f3cf-d6c4-4bd0-ac8c-de3d75ffa77e")]
    pub the_0706_f3_cf_d6_c4_4_bd0_ac8_c_de3_d75_ffa77_e: Option<i64>,

    #[serde(rename = "074b5e4a-84f8-428d-884e-4592a77ee061")]
    pub the_074_b5_e4_a_84_f8_428_d_884_e_4592_a77_ee061: Option<i64>,

    #[serde(rename = "09a77dd0-13c6-4c18-870a-63cd005ddff6")]
    pub the_09_a77_dd0_13_c6_4_c18_870_a_63_cd005_ddff6: Option<i64>,

    #[serde(rename = "0a449b4d-504b-448c-9516-6027fd6d216e")]
    pub the_0_a449_b4_d_504_b_448_c_95166027_fd6_d216_e: Option<i64>,

    #[serde(rename = "0b672007-ebfb-476d-8fdb-fb66bad78df2")]
    pub the_0_b672007_ebfb_476_d_8_fdb_fb66_bad78_df2: Option<i64>,

    #[serde(rename = "0eae657b-3592-43fb-93e1-b878680d2b53")]
    pub the_0_eae657_b_359243_fb_93_e1_b878680_d2_b53: Option<i64>,

    #[serde(rename = "105bc3ff-1320-4e37-8ef0-8d595cb95dd0")]
    pub the_105_bc3_ff_13204_e37_8_ef0_8_d595_cb95_dd0: Option<i64>,

    #[serde(rename = "110a62be-bc8a-4f0c-8046-0eb580b1af1c")]
    pub the_110_a62_be_bc8_a_4_f0_c_80460_eb580_b1_af1_c: Option<i64>,

    #[serde(rename = "11b92425-aa46-4691-b42f-baa2b6ddb541")]
    pub the_11_b92425_aa46_4691_b42_f_baa2_b6_ddb541: Option<i64>,

    #[serde(rename = "16be150e-372e-453e-b6ff-597aa42ca5ee")]
    pub the_16_be150_e_372_e_453_e_b6_ff_597_aa42_ca5_ee: Option<i64>,

    #[serde(rename = "16d1fd9b-c62b-4bed-b68a-b3a2d6e21524")]
    pub the_16_d1_fd9_b_c62_b_4_bed_b68_a_b3_a2_d6_e21524: Option<i64>,

    #[serde(rename = "19f81c84-9a94-49fa-9c23-2e1355126250")]
    pub the_19_f81_c84_9_a94_49_fa_9_c23_2_e1355126250: Option<i64>,

    #[serde(rename = "1a51664e-efec-45fa-b0ba-06d04c344628")]
    pub the_1_a51664_e_efec_45_fa_b0_ba_06_d04_c344628: Option<i64>,

    #[serde(rename = "1e04e5cc-80a6-41c0-af0d-7292817eed79")]
    pub the_1_e04_e5_cc_80_a6_41_c0_af0_d_7292817_eed79: Option<i64>,

    #[serde(rename = "22d8a1e9-e679-4bde-ae8a-318cb591d1c8")]
    pub the_22_d8_a1_e9_e679_4_bde_ae8_a_318_cb591_d1_c8: Option<i64>,

    #[serde(rename = "23a2cea4-5df7-4ed0-bb2c-b8c297518ada")]
    pub the_23_a2_cea4_5_df7_4_ed0_bb2_c_b8_c297518_ada: Option<i64>,

    #[serde(rename = "23e4cbc1-e9cd-47fa-a35b-bfa06f726cb7")]
    pub the_23_e4_cbc1_e9_cd_47_fa_a35_b_bfa06_f726_cb7: Option<i64>,

    #[serde(rename = "258f6389-aac1-43d2-b30a-4b4dde90d5eb")]
    pub the_258_f6389_aac1_43_d2_b30_a_4_b4_dde90_d5_eb: Option<i64>,

    #[serde(rename = "2957236a-6077-4012-a445-8c5be111afd0")]
    pub the_2957236_a_60774012_a445_8_c5_be111_afd0: Option<i64>,

    #[serde(rename = "2d02b60b-d858-4b8a-a835-c1e8fe1b8fe0")]
    pub the_2_d02_b60_b_d858_4_b8_a_a835_c1_e8_fe1_b8_fe0: Option<i64>,

    #[serde(rename = "2d07beca-bdb1-4ecb-bcfe-5913e2b406f5")]
    pub the_2_d07_beca_bdb1_4_ecb_bcfe_5913_e2_b406_f5: Option<i64>,

    #[serde(rename = "2dc7a1fa-3ae6-47ed-8c92-5d80167959f5")]
    pub the_2_dc7_a1_fa_3_ae6_47_ed_8_c92_5_d80167959_f5: Option<i64>,

    #[serde(rename = "2de5c38c-3d72-4d97-af0f-15e98eba2225")]
    pub the_2_de5_c38_c_3_d72_4_d97_af0_f_15_e98_eba2225: Option<i64>,

    #[serde(rename = "2e22beba-8e36-42ba-a8bf-975683c52b5f")]
    pub the_2_e22_beba_8_e36_42_ba_a8_bf_975683_c52_b5_f: Option<i64>,

    #[serde(rename = "2ec5927e-9905-408c-a04c-65a8879f846a")]
    pub the_2_ec5927_e_9905408_c_a04_c_65_a8879_f846_a: Option<i64>,

    #[serde(rename = "30c9bcd2-cc5a-421d-97d0-d39fefad053a")]
    pub the_30_c9_bcd2_cc5_a_421_d_97_d0_d39_fefad053_a: Option<i64>,

    #[serde(rename = "34a2e6ca-08fd-468e-894b-c707d6ce460a")]
    pub the_34_a2_e6_ca_08_fd_468_e_894_b_c707_d6_ce460_a: Option<i64>,

    #[serde(rename = "36569151-a2fb-43c1-9df7-2df512424c82")]
    pub the_36569151_a2_fb_43_c1_9_df7_2_df512424_c82: Option<i64>,

    #[serde(rename = "365b4517-4b0a-45da-aaa6-161dd77de99a")]
    pub the_365_b4517_4_b0_a_45_da_aaa6_161_dd77_de99_a: Option<i64>,

    #[serde(rename = "36f4efea-9d27-4457-a7b4-4b45ad2e23a3")]
    pub the_36_f4_efea_9_d27_4457_a7_b4_4_b45_ad2_e23_a3: Option<i64>,

    #[serde(rename = "378e3344-1a1a-4332-80cc-3da45954a4f4")]
    pub the_378_e3344_1_a1_a_433280_cc_3_da45954_a4_f4: Option<i64>,

    #[serde(rename = "3a094991-4cbc-4786-b74c-688876d243f4")]
    pub the_3_a094991_4_cbc_4786_b74_c_688876_d243_f4: Option<i64>,

    #[serde(rename = "3a4412d6-5404-4801-bf06-81cb7884fae4")]
    pub the_3_a4412_d6_54044801_bf06_81_cb7884_fae4: Option<i64>,

    #[serde(rename = "3aba36e6-9dd7-417f-9d3a-69d778439020")]
    pub the_3_aba36_e6_9_dd7_417_f_9_d3_a_69_d778439020: Option<i64>,

    #[serde(rename = "3b1c5a25-ed79-4ce2-87d4-0c1cf3ff342e")]
    pub the_3_b1_c5_a25_ed79_4_ce2_87_d4_0_c1_cf3_ff342_e: Option<i64>,

    #[serde(rename = "3cea1405-3a5f-432c-96c3-a85dc7f163ee")]
    pub the_3_cea1405_3_a5_f_432_c_96_c3_a85_dc7_f163_ee: Option<i64>,

    #[serde(rename = "3d858bda-dcef-4d05-928e-6557d3123f17")]
    pub the_3_d858_bda_dcef_4_d05_928_e_6557_d3123_f17: Option<i64>,

    #[serde(rename = "3f8bbb15-61c0-4e3f-8e4a-907a5fb1565e")]
    pub the_3_f8_bbb15_61_c0_4_e3_f_8_e4_a_907_a5_fb1565_e: Option<i64>,

    #[serde(rename = "444f2846-8927-4271-b2ca-6bf8b5b94610")]
    pub the_444_f2846_89274271_b2_ca_6_bf8_b5_b94610: Option<i64>,

    #[serde(rename = "44d9dc46-7e81-4e21-acff-c0f5dd399ae3")]
    pub the_44_d9_dc46_7_e81_4_e21_acff_c0_f5_dd399_ae3: Option<i64>,

    #[serde(rename = "46358869-dce9-4a01-bfba-ac24fc56f57e")]
    pub the_46358869_dce9_4_a01_bfba_ac24_fc56_f57_e: Option<i64>,

    #[serde(rename = "4b1004bc-345e-4084-8d18-b46315624864")]
    pub the_4_b1004_bc_345_e_40848_d18_b46315624864: Option<i64>,

    #[serde(rename = "4c192065-65d8-4010-8145-395f82d24ddf")]
    pub the_4_c192065_65_d8_40108145395_f82_d24_ddf: Option<i64>,

    #[serde(rename = "4cd14d96-f817-41a3-af6c-2d3ed0dd20b7")]
    pub the_4_cd14_d96_f817_41_a3_af6_c_2_d3_ed0_dd20_b7: Option<i64>,

    #[serde(rename = "4cf31f0e-fb42-4933-8fdb-fde58d109ced")]
    pub the_4_cf31_f0_e_fb42_49338_fdb_fde58_d109_ced: Option<i64>,

    #[serde(rename = "4d5f27b5-8924-498f-aa4c-7f5967c0c7c6")]
    pub the_4_d5_f27_b5_8924498_f_aa4_c_7_f5967_c0_c7_c6: Option<i64>,

    #[serde(rename = "505ae98b-7d85-4f51-99ef-60ccd7365d97")]
    pub the_505_ae98_b_7_d85_4_f51_99_ef_60_ccd7365_d97: Option<i64>,

    #[serde(rename = "5371833b-a620-4952-b2cb-a15eed8ad183")]
    pub the_5371833_b_a620_4952_b2_cb_a15_eed8_ad183: Option<i64>,

    #[serde(rename = "54d0d0f2-16e0-42a0-9fff-79cfa7c4a157")]
    pub the_54_d0_d0_f2_16_e0_42_a0_9_fff_79_cfa7_c4_a157: Option<i64>,

    #[serde(rename = "55c9fee3-79c8-4467-8dfb-ff1e340aae8c")]
    pub the_55_c9_fee3_79_c8_44678_dfb_ff1_e340_aae8_c: Option<i64>,

    #[serde(rename = "5663778c-c8fb-4408-908a-31dc1f6c55cc")]
    pub the_5663778_c_c8_fb_4408908_a_31_dc1_f6_c55_cc: Option<i64>,

    #[serde(rename = "5666fce7-3b39-4ade-9220-71244d9be5d8")]
    pub the_5666_fce7_3_b39_4_ade_922071244_d9_be5_d8: Option<i64>,

    #[serde(rename = "57d3f614-f8d3-4dfd-b486-075f823fdb0b")]
    pub the_57_d3_f614_f8_d3_4_dfd_b486_075_f823_fdb0_b: Option<i64>,

    #[serde(rename = "57ec08cc-0411-4643-b304-0e80dbc15ac7")]
    pub the_57_ec08_cc_04114643_b304_0_e80_dbc15_ac7: Option<i64>,

    #[serde(rename = "5818fb9b-f191-462e-9085-6fe311aaaf70")]
    pub the_5818_fb9_b_f191_462_e_90856_fe311_aaaf70: Option<i64>,

    #[serde(rename = "5d3bd8ab-cc9a-4aa5-bbcd-fa0f96566c64")]
    pub the_5_d3_bd8_ab_cc9_a_4_aa5_bbcd_fa0_f96566_c64: Option<i64>,

    #[serde(rename = "628bb28b-306e-4ff7-ad02-05524bcf246a")]
    pub the_628_bb28_b_306_e_4_ff7_ad02_05524_bcf246_a: Option<i64>,

    #[serde(rename = "635c332d-6ea9-4766-b391-ae4c3435f677")]
    pub the_635_c332_d_6_ea9_4766_b391_ae4_c3435_f677: Option<i64>,

    #[serde(rename = "6526d5df-6a9c-48e1-ba50-12dec0d8b22f")]
    pub the_6526_d5_df_6_a9_c_48_e1_ba50_12_dec0_d8_b22_f: Option<i64>,

    #[serde(rename = "6e655fc7-5190-4e55-99a0-89683d443cfc")]
    pub the_6_e655_fc7_51904_e55_99_a0_89683_d443_cfc: Option<i64>,

    #[serde(rename = "6f9ff34d-825f-477b-8600-1cec4febaecf")]
    pub the_6_f9_ff34_d_825_f_477_b_86001_cec4_febaecf: Option<i64>,

    #[serde(rename = "70167f08-5e85-44d7-b047-2961201c1615")]
    pub the_70167_f08_5_e85_44_d7_b047_2961201_c1615: Option<i64>,

    #[serde(rename = "71b157bc-8a50-4c05-a785-034f660e493f")]
    pub the_71_b157_bc_8_a50_4_c05_a785_034_f660_e493_f: Option<i64>,

    #[serde(rename = "747b8e4a-7e50-4638-a973-ea7950a3e739")]
    pub the_747_b8_e4_a_7_e50_4638_a973_ea7950_a3_e739: Option<i64>,

    #[serde(rename = "74aea6b6-34f9-48f4-b298-7345e1f9f7cb")]
    pub the_74_aea6_b6_34_f9_48_f4_b298_7345_e1_f9_f7_cb: Option<i64>,

    #[serde(rename = "75667373-b350-499b-b86e-5518b6f9f6ab")]
    pub the_75667373_b350_499_b_b86_e_5518_b6_f9_f6_ab: Option<i64>,

    #[serde(rename = "76d3489f-c7c4-4cb9-9c58-b1e1bab062d1")]
    pub the_76_d3489_f_c7_c4_4_cb9_9_c58_b1_e1_bab062_d1: Option<i64>,

    #[serde(rename = "780054a7-74ee-44fd-ab5f-6dd4637c5ef1")]
    pub the_780054_a7_74_ee_44_fd_ab5_f_6_dd4637_c5_ef1: Option<i64>,

    #[serde(rename = "79347640-8d5d-4e41-819d-2b0c86f20b76")]
    pub the_793476408_d5_d_4_e41_819_d_2_b0_c86_f20_b76: Option<i64>,

    #[serde(rename = "7966eb04-efcc-499b-8f03-d13916330531")]
    pub the_7966_eb04_efcc_499_b_8_f03_d13916330531: Option<i64>,

    #[serde(rename = "7ce9e0b0-9639-45f1-8db6-32c30ca0012d")]
    pub the_7_ce9_e0_b0_963945_f1_8_db6_32_c30_ca0012_d: Option<i64>,

    #[serde(rename = "7dc37924-0bb8-4e40-a826-c497d51e447c")]
    pub the_7_dc37924_0_bb8_4_e40_a826_c497_d51_e447_c: Option<i64>,

    #[serde(rename = "86435ed2-d205-4c37-be22-89683b9a7a62")]
    pub the_86435_ed2_d205_4_c37_be22_89683_b9_a7_a62: Option<i64>,

    #[serde(rename = "86f4485a-a6db-470b-82f5-e95e6b353537")]
    pub the_86_f4485_a_a6_db_470_b_82_f5_e95_e6_b353537: Option<i64>,

    #[serde(rename = "8756d8e1-bd9d-4116-8fd0-5ea06c2e80c3")]
    pub the_8756_d8_e1_bd9_d_41168_fd0_5_ea06_c2_e80_c3: Option<i64>,

    #[serde(rename = "878c1bf6-0d21-4659-bfee-916c8314d69c")]
    pub the_878_c1_bf6_0_d21_4659_bfee_916_c8314_d69_c: Option<i64>,

    #[serde(rename = "88151292-6c12-4fb8-b2d6-3e64821293b3")]
    pub the_881512926_c12_4_fb8_b2_d6_3_e64821293_b3: Option<i64>,

    #[serde(rename = "89796ffb-843a-4163-8dec-1bef229c68cb")]
    pub the_89796_ffb_843_a_41638_dec_1_bef229_c68_cb: Option<i64>,

    #[serde(rename = "8981c839-cbcf-47e3-a74e-8731dcff24fe")]
    pub the_8981_c839_cbcf_47_e3_a74_e_8731_dcff24_fe: Option<i64>,

    #[serde(rename = "8aaf714b-d42a-40b0-9165-384befc66d55")]
    pub the_8_aaf714_b_d42_a_40_b0_9165384_befc66_d55: Option<i64>,

    #[serde(rename = "8b38afb3-2e20-4e73-bb00-22bab14e3cda")]
    pub the_8_b38_afb3_2_e20_4_e73_bb00_22_bab14_e3_cda: Option<i64>,

    #[serde(rename = "8d7ba290-5f87-403c-81e3-cf5a2b6a6082")]
    pub the_8_d7_ba290_5_f87_403_c_81_e3_cf5_a2_b6_a6082: Option<i64>,

    #[serde(rename = "8d87c468-699a-47a8-b40d-cfb73a5660ad")]
    pub the_8_d87_c468_699_a_47_a8_b40_d_cfb73_a5660_ad: Option<i64>,

    #[serde(rename = "910974d7-cbfd-4d2d-8733-7e85759932da")]
    pub the_910974_d7_cbfd_4_d2_d_87337_e85759932_da: Option<i64>,

    #[serde(rename = "93e71a0e-80fc-46b7-beaf-d204c425fe03")]
    pub the_93_e71_a0_e_80_fc_46_b7_beaf_d204_c425_fe03: Option<i64>,

    #[serde(rename = "93f91157-f628-4c9a-a392-d2b1dbd79ac5")]
    pub the_93_f91157_f628_4_c9_a_a392_d2_b1_dbd79_ac5: Option<i64>,

    #[serde(rename = "9494152b-99f6-4adb-9573-f9e084bc813f")]
    pub the_9494152_b_99_f6_4_adb_9573_f9_e084_bc813_f: Option<i64>,

    #[serde(rename = "9657f04e-1c51-4951-88de-d376bb57f5bd")]
    pub the_9657_f04_e_1_c51_495188_de_d376_bb57_f5_bd: Option<i64>,

    #[serde(rename = "9685b9a9-8765-49e1-88ca-c153ad0276d0")]
    pub the_9685_b9_a9_876549_e1_88_ca_c153_ad0276_d0: Option<i64>,

    #[serde(rename = "979aee4a-6d80-4863-bf1c-ee1a78e06024")]
    pub the_979_aee4_a_6_d80_4863_bf1_c_ee1_a78_e06024: Option<i64>,

    #[serde(rename = "99edf2f4-f47d-46ee-998a-4cb4200236f7")]
    pub the_99_edf2_f4_f47_d_46_ee_998_a_4_cb4200236_f7: Option<i64>,

    #[serde(rename = "9a2f6bb9-c72c-437c-a3c4-e076dc5d10d4")]
    pub the_9_a2_f6_bb9_c72_c_437_c_a3_c4_e076_dc5_d10_d4: Option<i64>,

    #[serde(rename = "9da5c6b8-ccad-4bb5-b6b8-dc1d6b8ca6ed")]
    pub the_9_da5_c6_b8_ccad_4_bb5_b6_b8_dc1_d6_b8_ca6_ed: Option<i64>,

    #[serde(rename = "9debc64f-74b7-4ae1-a4d6-fce0144b6ea5")]
    pub the_9_debc64_f_74_b7_4_ae1_a4_d6_fce0144_b6_ea5: Option<i64>,

    #[serde(rename = "a01f0ade-0186-464d-8c68-e19a29cb66f0")]
    pub a01_f0_ade_0186464_d_8_c68_e19_a29_cb66_f0: Option<i64>,

    #[serde(rename = "a37f9158-7f82-46bc-908c-c9e2dda7c33b")]
    pub a37_f9158_7_f82_46_bc_908_c_c9_e2_dda7_c33_b: Option<i64>,

    #[serde(rename = "a554e084-b483-42da-89fb-39cd49ad7df6")]
    pub a554_e084_b483_42_da_89_fb_39_cd49_ad7_df6: Option<i64>,

    #[serde(rename = "a922603d-30c6-48d1-a83b-ae9be96675b6")]
    pub a922603_d_30_c6_48_d1_a83_b_ae9_be96675_b6: Option<i64>,

    #[serde(rename = "a94de6ef-5fc9-4470-89a0-557072fe4daf")]
    pub a94_de6_ef_5_fc9_447089_a0_557072_fe4_daf: Option<i64>,

    #[serde(rename = "adc5b394-8f76-416d-9ce9-813706877b84")]
    pub adc5_b394_8_f76_416_d_9_ce9_813706877_b84: Option<i64>,

    #[serde(rename = "ae0661b9-af66-4d4b-acc7-041e5cccb4bb")]
    pub ae0661_b9_af66_4_d4_b_acc7_041_e5_cccb4_bb: Option<i64>,

    #[serde(rename = "b024e975-1c4a-4575-8936-a3754a08806a")]
    pub b024_e975_1_c4_a_45758936_a3754_a08806_a: Option<i64>,

    #[serde(rename = "b069fdc6-2204-423a-932c-09037adcd845")]
    pub b069_fdc6_2204423_a_932_c_09037_adcd845: Option<i64>,

    #[serde(rename = "b1a50aa9-c515-46e8-8db9-d5378840362c")]
    pub b1_a50_aa9_c515_46_e8_8_db9_d5378840362_c: Option<i64>,

    #[serde(rename = "b320131f-da0d-43e1-9b98-f936a0ee417a")]
    pub b320131_f_da0_d_43_e1_9_b98_f936_a0_ee417_a: Option<i64>,

    #[serde(rename = "b35926d4-22a3-4419-8fab-686c41687055")]
    pub b35926_d4_22_a3_44198_fab_686_c41687055: Option<i64>,

    #[serde(rename = "b47df036-3aa4-4b98-8e9e-fe1d3ff1894b")]
    pub b47_df036_3_aa4_4_b98_8_e9_e_fe1_d3_ff1894_b: Option<i64>,

    #[serde(rename = "b5b4fb6b-08d8-401a-85d5-f08afa84af63")]
    pub b5_b4_fb6_b_08_d8_401_a_85_d5_f08_afa84_af63: Option<i64>,

    #[serde(rename = "b63be8c2-576a-4d6e-8daf-814f8bcea96f")]
    pub b63_be8_c2_576_a_4_d6_e_8_daf_814_f8_bcea96_f: Option<i64>,

    #[serde(rename = "b6b5df8f-5602-4883-b47d-07e77ed9d5af")]
    pub b6_b5_df8_f_56024883_b47_d_07_e77_ed9_d5_af: Option<i64>,

    #[serde(rename = "b72f3061-f573-40d7-832a-5ad475bd7909")]
    pub b72_f3061_f573_40_d7_832_a_5_ad475_bd7909: Option<i64>,

    #[serde(rename = "b7df2ea6-f4e8-4e6b-8c98-f730701f3717")]
    pub b7_df2_ea6_f4_e8_4_e6_b_8_c98_f730701_f3717: Option<i64>,

    #[serde(rename = "b7f9cc0c-6a6c-4bed-adbb-2d2d2dfbe810")]
    pub b7_f9_cc0_c_6_a6_c_4_bed_adbb_2_d2_d2_dfbe810: Option<i64>,

    #[serde(rename = "ba6d5599-1242-41ed-be64-90de7b1c255f")]
    pub ba6_d5599_124241_ed_be64_90_de7_b1_c255_f: Option<i64>,

    #[serde(rename = "bb4a9de5-c924-4923-a0cb-9d1445f1ee5d")]
    pub bb4_a9_de5_c924_4923_a0_cb_9_d1445_f1_ee5_d: Option<i64>,

    #[serde(rename = "bebf13f9-82d1-4133-9b14-4a96de029ccf")]
    pub bebf13_f9_82_d1_41339_b14_4_a96_de029_ccf: Option<i64>,

    #[serde(rename = "bfd38797-8404-4b38-8b82-341da28b1f83")]
    pub bfd38797_84044_b38_8_b82_341_da28_b1_f83: Option<i64>,

    #[serde(rename = "c0dc2c80-463e-49f7-9e00-c62473d677c8")]
    pub c0_dc2_c80_463_e_49_f7_9_e00_c62473_d677_c8: Option<i64>,

    #[serde(rename = "c19bb50b-9a22-4dd2-8200-bce639b1b239")]
    pub c19_bb50_b_9_a22_4_dd2_8200_bce639_b1_b239: Option<i64>,

    #[serde(rename = "c73b705c-40ad-4633-a6ed-d357ee2e2bcf")]
    pub c73_b705_c_40_ad_4633_a6_ed_d357_ee2_e2_bcf: Option<i64>,

    #[serde(rename = "c794d5aa-6104-420e-ae6f-3b2c270253fd")]
    pub c794_d5_aa_6104420_e_ae6_f_3_b2_c270253_fd: Option<i64>,

    #[serde(rename = "ca117809-cda1-4ae0-b607-53079fb5b133")]
    pub ca117809_cda1_4_ae0_b607_53079_fb5_b133: Option<i64>,

    #[serde(rename = "ca3f1c8c-c025-4d8e-8eef-5be6accbeb16")]
    pub ca3_f1_c8_c_c025_4_d8_e_8_eef_5_be6_accbeb16: Option<i64>,

    #[serde(rename = "cade1731-39a8-43f3-be8e-d2302711fe8b")]
    pub cade1731_39_a8_43_f3_be8_e_d2302711_fe8_b: Option<i64>,

    #[serde(rename = "cbd44c06-231a-4d1a-bb7d-4170b06e566a")]
    pub cbd44_c06_231_a_4_d1_a_bb7_d_4170_b06_e566_a: Option<i64>,

    #[serde(rename = "cc9de838-4431-4cc7-9c3e-15b15b2142b0")]
    pub cc9_de838_44314_cc7_9_c3_e_15_b15_b2142_b0: Option<i64>,

    #[serde(rename = "cd29d13d-99d4-414b-8faa-f0819b2de526")]
    pub cd29_d13_d_99_d4_414_b_8_faa_f0819_b2_de526: Option<i64>,

    #[serde(rename = "d0762a7e-004b-48a9-a832-a993982b305b")]
    pub d0762_a7_e_004_b_48_a9_a832_a993982_b305_b: Option<i64>,

    #[serde(rename = "d2874e7f-8e88-442a-a176-e256df68a49b")]
    pub d2874_e7_f_8_e88_442_a_a176_e256_df68_a49_b: Option<i64>,

    #[serde(rename = "d2949bd0-6a28-4e0d-aa07-cecc437cbd99")]
    pub d2949_bd0_6_a28_4_e0_d_aa07_cecc437_cbd99: Option<i64>,

    #[serde(rename = "d2c33336-b5a9-4ce1-86bb-f376ec66efbd")]
    pub d2_c33336_b5_a9_4_ce1_86_bb_f376_ec66_efbd: Option<i64>,

    #[serde(rename = "d6a352fc-b675-40a0-864d-f4fd50aaeea0")]
    pub d6_a352_fc_b675_40_a0_864_d_f4_fd50_aaeea0: Option<i64>,

    #[serde(rename = "d82a1a80-dff3-4767-bab6-484b2eb7aee1")]
    pub d82_a1_a80_dff3_4767_bab6_484_b2_eb7_aee1: Option<i64>,

    #[serde(rename = "d9f89a8a-c563-493e-9d64-78e4f9a55d4a")]
    pub d9_f89_a8_a_c563_493_e_9_d64_78_e4_f9_a55_d4_a: Option<i64>,

    #[serde(rename = "e11df0cc-3a95-4159-9a84-fecbbf23ae05")]
    pub e11_df0_cc_3_a95_41599_a84_fecbbf23_ae05: Option<i64>,

    #[serde(rename = "e12313fe-c0c9-49de-9d11-8b7408aa92ce")]
    pub e12313_fe_c0_c9_49_de_9_d11_8_b7408_aa92_ce: Option<i64>,

    #[serde(rename = "e4f7549c-17af-4e35-b89b-f0fae855a31b")]
    pub e4_f7549_c_17_af_4_e35_b89_b_f0_fae855_a31_b: Option<i64>,

    #[serde(rename = "ea3c8019-b6b6-4830-b952-7e9c2ce707bd")]
    pub ea3_c8019_b6_b6_4830_b952_7_e9_c2_ce707_bd: Option<i64>,

    #[serde(rename = "eb67ae5e-c4bf-46ca-bbbc-425cd34182ff")]
    pub eb67_ae5_e_c4_bf_46_ca_bbbc_425_cd34182_ff: Option<i64>,

    #[serde(rename = "ee722cbd-812f-4525-81d7-dfa89fb867a4")]
    pub ee722_cbd_812_f_452581_d7_dfa89_fb867_a4: Option<i64>,

    #[serde(rename = "effdbd8d-a54f-4049-a3c8-b5f944e5278b")]
    pub effdbd8_d_a54_f_4049_a3_c8_b5_f944_e5278_b: Option<i64>,

    #[serde(rename = "f02aeae2-5e6a-4098-9842-02d2273f25c7")]
    pub f02_aeae2_5_e6_a_4098984202_d2273_f25_c7: Option<i64>,

    #[serde(rename = "f0ec8435-0427-4ffd-ad0c-a67f60a75e0e")]
    pub f0_ec8435_04274_ffd_ad0_c_a67_f60_a75_e0_e: Option<i64>,

    #[serde(rename = "f3490435-a42f-42a8-ab89-d59e8dc8d599")]
    pub f3490435_a42_f_42_a8_ab89_d59_e8_dc8_d599: Option<i64>,

    #[serde(rename = "f8d99dc7-ae37-4f35-b08c-543864a347f2")]
    pub f8_d99_dc7_ae37_4_f35_b08_c_543864_a347_f2: Option<i64>,

    #[serde(rename = "f9045b82-5570-43d4-856b-bed5095515c6")]
    pub f9045_b82_557043_d4_856_b_bed5095515_c6: Option<i64>,

    #[serde(rename = "fab9420f-0730-4054-bd17-355113f204c2")]
    pub fab9420_f_07304054_bd17_355113_f204_c2: Option<i64>,

    #[serde(rename = "fca16c92-5f03-45b9-abbe-760866878ffe")]
    pub fca16_c92_5_f03_45_b9_abbe_760866878_ffe: Option<i64>,
}
