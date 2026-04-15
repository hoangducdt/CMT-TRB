const helpsList = [
    {
        name: "Fail Data Tool",
        category: "tools",
        image: FailDataTool,
        description: "....",
        navto: "faildata",
        guide: {
            note: 'Chỉ áp dụng tại CMT-TRB.',
            steps: [
                `Mở "Remote Desktop" và tiến hành remote vào "sssapprdsnlb.ss.intel.com"<br><img src="${faildataguide1}">`,
                `Mở "File Explorer" và vào thư mục "\\\\ssfile1.ss.intel.com\\cmtdata\\prod"<br><img src="${faildataguide2}">`,
                `Ở ô tìm kiếm của thư mục prod nhập Lot_Operation<br><img src="${faildataguide3}">`,
                'Sao chép thư mục tìm được về máy',
                `Tại giao diện chính của công cụ bấm "Chọn Tệp Dữ Liệu Test" và chọn các Sum cần xem.<br><img src="${faildataguide4}">`,
                `Bấm "Tìm Fail Data" và lựa chọn TIU/Site/Bin bạn cần xem<br><img src="${faildataguide5}">`,
                `Bấm "Xem Pinmap" để xem vị trí các pin fail đã được hight light sẵn trên pinmap<br><img src="${faildataguide6}">`,
                'Cần bổ sung thêm thông tin, vui lòng lên hệ tool owner'
            ]
        }
    },
    {
        name: "Pinmap Tool",
        category: "tools",
        image: PinmapTool,
        description: "....",
        navto: "pinmaptool",
        guide: {
            note: 'Chỉ áp dụng tại CMT-TRB.',
            steps: [
                `Nhấp vào nút "Chọn tệp CRES/MTP" và chọn tệp cần<br><img src="${pinmaptoolguide1}">`,
                `Bấm "Phân tích tệp" sau đó chọn tệp cần xem<br><img src="${pinmaptoolguide2}">`,
                `Bấm "Xem pinmap để xem pinmap vừa được vẽ"<br><img src="${pinmaptoolguide3}">`,
                'Cần bổ sung thêm thông tin, vui lòng lên hệ tool owner'
            ]
        }
    },
    {
        name: "Pin Finder Helper",
        category: "tools",
        image: PinFinderHelper,
        description: "....",
        navto: "pinfinderhelper",
        guide: {
            note: 'Chỉ áp dụng tại CMT-TRB.',
            steps: [
                `Sao chép danh sách pin từ web pin<br><img src="${pinfinderhelperguide1}">`,
                `Dán vào ô input và bấm "Phân tích DS pin"<br><img src="${pinfinderhelperguide2}">`,
                `Chọn sản phẩm và bấm "Xem pinmap để xem pinmap vừa được vẽ"<br><img src="${pinfinderhelperguide3}">`,
                'Cần bổ sung thêm thông tin, vui lòng lên hệ tool owner'
            ]
        }
    },
    {
        name: "Channel Finder",
        category: "tools",
        image: ChannelFinder,
        description: "....",
        navto: "channelfinder",
        guide: {
            note: 'Chỉ áp dụng tại CMT-TRB.',
            steps: [
                `Chọn Sản phẩm/Site/Channel cần xem và bấm "Phân tích channel"<br><img src="${channelfinderguide1}">`,
                `Bấm "Xem pinmap để xem pinmap vừa được vẽ"<br><img src="${channelfinderguide2}">`,
                'Cần bổ sung thêm thông tin, vui lòng lên hệ tool owner'
            ]
        }
    },
    {
        name: "Pin Finder",
        category: "tools",
        image: PinFinder,
        description: "....",
        navto: "pinfinder",
        guide: {
            note: 'Chỉ áp dụng tại CMT-TRB.',
            steps: [
                `Chọn Sản phẩm/Site/Pin cần xem và bấm "Phân tích channel"<br><img src="${pinfinderguide1}">`,
                `Bấm "Xem pinmap để xem pinmap vừa được vẽ"<br><img src="${pinfinderguide2}">`,
                'Cần bổ sung thêm thông tin, vui lòng lên hệ tool owner'
            ]
        }
    },
];