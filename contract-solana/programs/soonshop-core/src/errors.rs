/**
 * SoonShop核心智能合约错误处理模块
 * 
 * 定义了所有可能的错误类型，便于调试和问题排查
 * 错误码按模块分类，每个模块预留100个错误码空间
 */

use anchor_lang::prelude::*;

#[error_code]
pub enum SoonShopError {
    // ================================
    // 通用错误 (6000-6099)
    // ================================
    
    #[msg("未授权的操作")]
    Unauthorized = 6000,
    
    #[msg("无效的参数")]
    InvalidParameter = 6001,
    
    #[msg("数学运算溢出")]
    MathOverflow = 6002,
    
    #[msg("数学运算下溢")]
    MathUnderflow = 6003,
    
    #[msg("除零错误")]
    DivisionByZero = 6004,
    
    #[msg("账户已初始化")]
    AlreadyInitialized = 6005,
    
    #[msg("账户未初始化")]
    NotInitialized = 6006,
    
    #[msg("账户类型不匹配")]
    AccountTypeMismatch = 6007,
    
    #[msg("账户所有者不匹配")]
    AccountOwnerMismatch = 6008,
    
    #[msg("签名验证失败")]
    SignatureVerificationFailed = 6009,
    
    #[msg("时间戳无效")]
    InvalidTimestamp = 6010,
    
    #[msg("操作超时")]
    OperationTimeout = 6011,
    
    #[msg("数据序列化失败")]
    SerializationError = 6012,
    
    #[msg("数据反序列化失败")]
    DeserializationError = 6013,
    
    #[msg("字符串长度超限")]
    StringTooLong = 6014,
    
    #[msg("数组长度超限")]
    ArrayTooLong = 6015,
    
    #[msg("算术运算溢出")]
    ArithmeticOverflow = 6016,
    
    #[msg("用户名格式无效")]
    InvalidUsername = 6017,
    
    #[msg("URL格式无效")]
    InvalidUrl = 6018,
    
    #[msg("电子邮件格式无效")]
    InvalidEmail = 6019,
    
    #[msg("价格数值无效")]
    InvalidPrice = 6020,
    
    #[msg("链式层级无效")]
    InvalidChainLevel = 6021,
    
    #[msg("质量评分无效")]
    InvalidQualityScore = 6022,
    
    #[msg("金额无效")]
    InvalidAmount = 6023,

    // ================================
    // 平台管理错误 (6100-6199)
    // ================================
    
    #[msg("平台未初始化")]
    PlatformNotInitialized = 6100,
    
    #[msg("平台已暂停")]
    PlatformPaused = 6101,
    
    #[msg("平台处于紧急状态")]
    PlatformInEmergency = 6102,
    
    #[msg("平台正在升级")]
    PlatformUpgrading = 6103,
    
    #[msg("平台配置无效")]
    InvalidPlatformConfig = 6104,
    
    #[msg("管理员权限不足")]
    InsufficientAdminPrivilege = 6105,
    
    #[msg("超级管理员权限不足")]
    InsufficientSuperAdminPrivilege = 6106,
    
    #[msg("平台版本不兼容")]
    IncompatiblePlatformVersion = 6107,
    
    #[msg("平台费率无效")]
    InvalidFeeRate = 6108,
    
    #[msg("版本号无效")]
    InvalidVersion = 6109,
    
    #[msg("不能将超级管理员添加为普通管理员")]
    CannotAddSuperAdminAsAdmin = 6110,
    
    #[msg("管理员数量已达上限")]
    TooManyAdmins = 6111,
    
    #[msg("管理员已存在")]
    AdminAlreadyExists = 6112,
    
    #[msg("管理员不存在")]
    AdminNotFound = 6113,
    
    #[msg("平台未激活")]
    PlatformNotActive = 6114,
    
    #[msg("平台处于紧急暂停状态")]
    PlatformEmergencyPaused = 6115,
    
    #[msg("今日紧急暂停次数已达上限")]
    TooManyEmergencyPauses = 6116,
    
    #[msg("无法移除主权限账户")]
    CannotRemoveMainAuthority = 6117,
    
    #[msg("必须至少有一个管理员")]
    MustHaveAtLeastOneAdmin = 6118,
    
    #[msg("系统已暂停")]
    AlreadyPaused = 6119,
    
    #[msg("系统未暂停")]
    NotPaused = 6120,
    
    #[msg("紧急暂停期间无法取消暂停")]
    CannotUnpauseDuringEmergency = 6121,
    
    #[msg("系统暂停中")]
    SystemPaused = 6122,

    // ================================
    // 用户管理错误 (6200-6299)
    // ================================
    
    #[msg("用户不存在")]
    UserNotFound = 6200,
    
    #[msg("用户未验证")]
    UserNotVerified = 6201,
    
    #[msg("用户已被暂停")]
    UserSuspended = 6202,
    
    #[msg("用户类型不匹配")]
    UserTypeMismatch = 6203,
    
    #[msg("生产者未认证")]
    ProducerNotVerified = 6204,
    
    #[msg("消费者未认证")]
    ConsumerNotVerified = 6205,
    
    #[msg("评估员未认证")]
    EvaluatorNotVerified = 6206,
    
    #[msg("用户信誉评分过低")]
    UserReputationTooLow = 6207,
    
    #[msg("用户操作频率过高")]
    UserOperationTooFrequent = 6208,

    // ================================
    // 提货券相关错误 (6300-6399)
    // ================================
    
    #[msg("提货券不存在")]
    VoucherNotFound = 6300,
    
    #[msg("提货券已过期")]
    VoucherExpired = 6301,
    
    #[msg("提货券数量不足")]
    InsufficientVoucherQuantity = 6302,
    
    #[msg("提货券状态无效")]
    InvalidVoucherStatus = 6303,
    
    #[msg("提货券类型不支持")]
    UnsupportedVoucherType = 6304,
    
    #[msg("提货券配置无效")]
    InvalidVoucherConfig = 6305,
    
    #[msg("超出获取限制")]
    ExceedsClaimLimit = 6306,
    
    #[msg("不满足获取条件")]
    ClaimConditionNotMet = 6307,
    
    #[msg("地理位置限制")]
    GeographicRestriction = 6308,
    
    #[msg("时间限制")]
    TimeRestriction = 6309,
    
    #[msg("用户类型限制")]
    UserTypeRestriction = 6310,
    
    #[msg("提货券已被锁定")]
    VoucherLocked = 6311,
    
    #[msg("提货券已被消费")]
    VoucherAlreadyConsumed = 6312,
    
    #[msg("提货券创建失败")]
    VoucherCreationFailed = 6313,

    // ================================
    // 消费相关错误 (6400-6499)
    // ================================
    
    #[msg("消费记录不存在")]
    ConsumptionNotFound = 6400,
    
    #[msg("消费证明无效")]
    InvalidConsumptionProof = 6401,
    
    #[msg("消费数量无效")]
    InvalidConsumptionQuantity = 6402,
    
    #[msg("消费状态无效")]
    InvalidConsumptionStatus = 6403,
    
    #[msg("消费已完成")]
    ConsumptionAlreadyCompleted = 6404,
    
    #[msg("消费未确认")]
    ConsumptionNotConfirmed = 6405,
    
    #[msg("消费争议中")]
    ConsumptionDisputed = 6406,
    
    #[msg("消费地点无效")]
    InvalidConsumptionLocation = 6407,
    
    #[msg("消费时间无效")]
    InvalidConsumptionTime = 6408,

    // ================================
    // 倍增奖励错误 (6500-6599)
    // ================================
    
    #[msg("倍增系数无效")]
    InvalidMultiplier = 6500,
    
    #[msg("倍增奖励计算失败")]
    MultiplierCalculationFailed = 6501,
    
    #[msg("奖励分发失败")]
    RewardDistributionFailed = 6502,
    
    #[msg("奖励额度不足")]
    InsufficientRewardBalance = 6503,
    
    #[msg("资金不足")]
    InsufficientFunds = 6504,
    
    #[msg("职工分红比例无效")]
    InvalidWorkerShareRatio = 6507,
    
    #[msg("链式倍增传递失败")]
    ChainMultiplierPropagationFailed = 6505,
    
    #[msg("上游消费信息无效")]
    InvalidUpstreamConsumption = 6506,

    // ================================
    // 企业评估错误 (6600-6699)
    // ================================
    
    #[msg("评估记录不存在")]
    EvaluationNotFound = 6600,
    
    #[msg("评估分数无效")]
    InvalidEvaluationScore = 6601,
    
    #[msg("评估详情无效")]
    InvalidEvaluationDetails = 6602,
    
    #[msg("评估员权限不足")]
    InsufficientEvaluatorPermission = 6603,
    
    #[msg("评估周期重复")]
    DuplicateEvaluationPeriod = 6604,
    
    #[msg("评估尚未批准")]
    EvaluationNotApproved = 6605,
    
    #[msg("企业评估已存在")]
    EvaluationAlreadyExists = 6606,
    
    #[msg("评估标准不匹配")]
    EvaluationCriteriaMismatch = 6607,

    // ================================
    // 价格监控错误 (6700-6799)
    // ================================
    
    #[msg("价格数据无效")]
    InvalidPriceData = 6700,
    
    #[msg("价格波动异常")]
    AbnormalPriceVolatility = 6701,
    
    #[msg("疑似价格操纵")]
    SuspectedPriceManipulation = 6702,
    
    #[msg("价格预言机故障")]
    PriceOracleMalfunction = 6703,
    
    #[msg("价格数据过时")]
    OutdatedPriceData = 6704,
    
    #[msg("价格监控暂停")]
    PriceMonitoringPaused = 6705,
    
    #[msg("通胀率超标")]
    InflationRateExceeded = 6706,

    // ================================
    // 应用场景错误 (6800-6899)
    // ================================
    
    #[msg("B2C订单信息无效")]
    InvalidB2COrderInfo = 6800,
    
    #[msg("餐厅预约信息无效")]
    InvalidRestaurantReservation = 6801,
    
    #[msg("医疗预约信息无效")]
    InvalidHealthcareAppointment = 6802,
    
    #[msg("住房租赁信息无效")]
    InvalidHousingRental = 6803,
    
    #[msg("教育报名信息无效")]
    InvalidEducationEnrollment = 6804,
    
    #[msg("服务提供方未认证")]
    ServiceProviderNotVerified = 6805,
    
    #[msg("服务时间冲突")]
    ServiceTimeConflict = 6806,
    
    #[msg("服务容量已满")]
    ServiceCapacityFull = 6807,
    
    #[msg("服务区域限制")]
    ServiceAreaRestriction = 6808,

    // ================================
    // Token相关错误 (6900-6999)
    // ================================
    
    #[msg("Token账户无效")]
    InvalidTokenAccount = 6900,
    
    #[msg("Token余额不足")]
    InsufficientTokenBalance = 6901,
    
    #[msg("Token转账失败")]
    TokenTransferFailed = 6902,
    
    #[msg("Token铸造失败")]
    TokenMintFailed = 6903,
    
    #[msg("Token销毁失败")]
    TokenBurnFailed = 6904,
    
    #[msg("Token权限不足")]
    InsufficientTokenAuthority = 6905,
    
    #[msg("Token冻结")]
    TokenFrozen = 6906,
} 