use enum_primitive::FromPrimitive;

enum_from_primitive! {
    #[derive(PartialEq)]
    pub enum QMetaTypeId {
        Void=43,
        Bool=1,
        Int=2,
        UInt=3,
        Double=6,
        QChar=7,
        QString=10,
        QByteArray=12,
        VoidStar=31,
        Long=32,
        LongLong=4,
        Short=33,
        Char=34,
        ULong=35,
        ULongLong=5,
        UShort=36,
        SChar=40,
        UChar=37,
        Float=38,
        QObjectStar=39,
        QVariant=41,
        QCursor=74,
        QDate=14,
        QSize=21,
        QTime=15,
        QVariantList=9,
        QPolygon=71,
        QPolygonF=86,
        QColor=67,
        QSizeF=22,
        QRectF=20,
        QLine=23,
        QTextLength=77,
        QStringList=11,
        QVariantMap=8,
        QVariantHash=28,
        QIcon=69,
        QPen=76,
        QLineF=24,
        QTextFormat=78,
        QRect=19,
        QPoint=25,
        QUrl=17,
        QRegExp=27,
        QRegularExpression=44,
        QDateTime=16,
        QPointF=26,
        QPalette=68,
        QFont=64,
        QBrush=66,
        QRegion=72,
        QBitArray=13,
        QImage=70,
        QKeySequence=75,
        QSizePolicy=121,
        QPixmap=65,
        QLocale=18,
        QBitmap=73,
        QMatrix=79,
        QTransform=80,
        QMatrix4x4=81,
        QVector2D=82,
        QVector3D=83,
        QVector4D=84,
        QQuaternion=85,
        QEasingCurve=29,
        QJsonValue=45,
        QJsonObject=46,
        QJsonArray=47,
        QJsonDocument=48,
        QModelIndex=42,
        QPersistentModelIndex=50,
        QUuid=30,
        QByteArrayList=49,
        User=1024,
        UnknownType=0,
    }
}

impl From<i32> for QMetaTypeId {
    fn from(val: i32) -> QMetaTypeId {
        QMetaTypeId::from_i32(val).unwrap_or(QMetaTypeId::UnknownType)
    }
}
impl From<QMetaTypeId> for i32 {
    fn from(val: QMetaTypeId) -> i32 {
        val as i32
    }
}

