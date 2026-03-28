import { BASE_URL } from "@/constants"

export const landingData = {
  hero: {
    titlePrefix: "SakaPOS — Kasir Digital untuk",
    rotatingWords: ["UMKM", "Retail", "Resto", "mu!"],
    description:
      "Hemat waktu, kurangi kesalahan, dan tingkatkan keuntungan dengan SakaPOS.",
    primaryCta: {
      label: "Buat Akun",
      href: "/register",
    },
    secondaryCta: {
      label: "Konsultasi Dulu",
      href: "##",
    },
    reviewLabel: "Review 4.8/5.0",
    previewAssets: ["/user-1.webp", "/user-1.webp", "/user-1.webp"],
    heroImage: "/hero-device.png",
  },
  features: {
    tagline: "Fitur Unggulan",
    title: {
      1: "Mengapa Harus Memilih ",
      2: "SakaPOS",
      3: " Untuk Bisnis Anda?",
    },
    description:
      "SakaPOS hadir untuk membantu bisnis Anda bekerja lebih cepat, lebih efisien, dan lebih cerdas. Dengan teknologi modern dan antarmuka yang mudah digunakan, SakaPOS tidak hanya sekadar sistem kasir—tetapi solusi lengkap untuk mengelola dan mengembangkan usaha Anda.",
    feature_list: [
      {
        icon: "monitor",
        title: "Antarmuka Mudah & Intuitif",
        description:
          "Didesain agar siapa pun dapat langsung mengoperasikan tanpa perlu pelatihan rumit.",
      },
      {
        icon: "package",
        title: "Manajemen Stok Otomatis",
        description:
          "Pantau ketersediaan produk secara real-time, dapatkan notifikasi saat stok menipis, dan hindari kehilangan penjualan.",
      },
      {
        icon: "bar-chart-3",
        title: "Laporan Penjualan Instan",
        description:
          "Akses laporan detail harian, mingguan, hingga bulanan hanya dengan sekali klik untuk keputusan bisnis yang lebih tepat.",
      },
      {
        icon: "users",
        title: "Multi-Outlet & Multi-User",
        description:
          "Kelola banyak cabang dan pengguna dalam satu sistem, dengan kontrol akses yang fleksibel.",
      },
      {
        icon: "credit-card",
        title: "Terintegrasi Pembayaran Digital",
        description:
          "Dukung pembayaran QRIS, kartu debit/kredit, dan e-wallet untuk memudahkan pelanggan.",
      },
      {
        icon: "cloud",
        title: "Cloud-Based & Aman",
        description:
          "Data bisnis tersimpan aman di cloud, bisa diakses kapan saja dan di mana saja.",
      },
    ],
  },
  benefits: {
    tagline: "Benefit",
    title: "Dampak Nyata untuk Bisnis Anda",
    description:
      "Dengan SakaPOS, Anda tidak hanya mendapatkan sistem kasir digital, tetapi juga solusi yang membantu bisnis Anda tumbuh lebih cepat, lebih efisien, dan lebih menguntungkan.",
    benefit_list: [
      {
        image: "/svg/supermarket-workers.svg",
        title: "Kontrol Penuh atas Penjualan & Stok",
        description:
          "Semua transaksi dan persediaan tercatat otomatis, sehingga Anda selalu tahu kondisi bisnis tanpa harus menebak-nebak.",
        points: [
          {
            icon: "badge-check",
            text: "Lacak produk paling laris dan stok yang hampir habis.",
          },
          {
            icon: "badge-check",
            text: "Cegah kerugian akibat barang hilang atau tercatat ganda.",
          },
          {
            icon: "badge-check",
            text: "Dapatkan laporan penjualan lengkap dalam hitungan detik.",
          },
        ],
      },
      {
        image: "/svg/time-management.svg",
        title: "Operasional Lebih Efisien",
        description:
          "Kurangi pekerjaan manual yang memakan waktu. Dengan SakaPOS, semua proses lebih cepat, praktis, dan minim kesalahan.",
        points: [
          {
            icon: "zap",
            text: "Transaksi lebih singkat dengan antarmuka sederhana.",
          },
          {
            icon: "printer",
            text: "Cetak struk digital maupun fisik secara otomatis.",
          },
          {
            icon: "repeat",
            text: "Kurangi input data berulang dengan integrasi stok & laporan.",
          },
        ],
      },
      {
        image: "/svg/analysis.svg",
        title: "Dorong Pertumbuhan Bisnis Anda",
        description:
          "Dengan data yang akurat dan sistem yang andal, Anda bisa fokus pada strategi untuk mengembangkan bisnis.",
        points: [
          {
            icon: "bar-chart-3",
            text: "Analisis performa cabang dan karyawan dengan mudah.",
          },
          {
            icon: "target",
            text: "Tentukan strategi promosi berbasis data, bukan asumsi.",
          },
          {
            icon: "store",
            text: "Buka peluang ekspansi dengan dukungan multi-outlet.",
          },
        ],
      },
    ],
  },
  integrations: {
    tagline: "Integrasi Pembayaran",
    title: "Dukung Semua Metode Pembayaran Digital",
    description:
      "SakaPOS memudahkan bisnis Anda menerima pembayaran digital secara lengkap dan terintegrasi. Mulai dari QRIS yang praktis hingga e-wallet populer seperti GoPay, OVO, Dana, ShopeePay, LinkAja, dan Doku. semua bisa digunakan pelanggan dengan sekali scan atau sentuhan. Setiap transaksi tercatat otomatis dalam sistem, aman, dan transparan tanpa proses manual. Dengan dukungan pembayaran digital yang luas, pelanggan lebih leluasa memilih cara bayar favorit mereka, sementara Anda menikmati pencatatan yang akurat, laporan yang rapi, dan alur kas yang lebih cepat.",
    integration_list: [
      {
        logo: "/qris.png",
        title: "QRIS",
        description:
          "Satu kode QR untuk semua pembayaran digital, mendukung transfer bank dan dompet digital.",
      },
      {
        logo: "/gopay.png",
        title: "GoPay",
        description:
          "Populer di kalangan anak muda dan pengguna Gojek, cocok untuk target pasar retail & F&B.",
      },
      {
        logo: "/shopeepay.png",
        title: "ShopeePay",
        description:
          "Pilihan favorit pengguna Shopee, cepat, praktis, dan sering menghadirkan promo cashback.",
      },
      {
        logo: "/dana.png",
        title: "Dana",
        description:
          "Mudah digunakan dengan fitur top-up praktis, banyak digunakan untuk pembayaran sehari-hari.",
      },
      {
        logo: "/ovo.png",
        title: "OVO",
        description:
          "E-wallet serbaguna dengan jaringan luas, memudahkan pelanggan bayar di mana saja.",
      },
      {
        logo: "/card.png",
        title: "Kartu Debit & Kredit",
        description:
          "Proses pembayaran kartu debit & kredit lebih cepat, aman, dan langsung terintegrasi dengan laporan penjualan.",
      },
    ],
  },
  faqs: {
    tagline: "FAQ",
    title: "Pertanyaan yang Sering Ditanyakan",
    faq_list: [
      {
        question: "Apa saja fitur utama yang tersedia di SakaPOS?",
        answer:
          "SakaPOS dilengkapi dengan manajemen stok otomatis, laporan penjualan instan, dukungan multi-outlet, serta integrasi pembayaran digital seperti QRIS, e-wallet, dan kartu.",
      },
      {
        question: "Apakah SakaPOS mudah digunakan?",
        answer:
          "Ya, SakaPOS dirancang dengan antarmuka yang sederhana dan intuitif, sehingga dapat digunakan oleh siapa saja tanpa perlu pelatihan rumit.",
      },
      {
        question: "Bagaimana cara mengakses laporan penjualan?",
        answer:
          "Anda dapat melihat laporan harian, mingguan, bulanan, bahkan laporan berdasarkan cabang, karyawan, produk, maupun pelanggan langsung dari dashboard SakaPOS.",
      },
      {
        question: "Apakah SakaPOS bisa dipakai untuk banyak cabang?",
        answer:
          "Tentu, SakaPOS mendukung multi-outlet. Semua cabang bisa dipantau dan dikelola dalam satu akun dengan akses yang dapat diatur sesuai kebutuhan.",
      },
      {
        question: "Apakah data saya aman?",
        answer:
          "Ya, semua data penjualan dan stok tersimpan aman di cloud dengan sistem enkripsi dan backup otomatis, sehingga Anda tidak perlu khawatir kehilangan data.",
      },
      {
        question: "Bisakah saya mencoba SakaPOS terlebih dahulu?",
        answer:
          "Ya, Anda dapat membuat akun gratis atau mengajukan demo untuk mencoba fitur SakaPOS sebelum memutuskan berlangganan.",
      },
    ],
  },
  pricing: {
    tagline: "Pilih Paket Sesuai Kebutuhan",
    title: "Harga Transparan, Solusi Fleksibel",
    description:
      "Kami menyediakan berbagai paket Point of Sale (POS) yang bisa disesuaikan dengan kebutuhan bisnis Anda. Mulai dari usaha kecil hingga perusahaan besar, semua bisa menggunakan sistem kami untuk mempermudah operasional, pencatatan penjualan, hingga integrasi pembayaran digital.",
    price_list: [
      {
        name: "BASIC",
        price: "3000000",
        tagline: "Solusi POS Sederhana untuk Bisnis Kecil",
        title: "POS Basic",
        description:
          "Cocok untuk UMKM atau usaha kecil yang baru mulai digitalisasi kasir.",
        features: [
          "Aplikasi POS berbasis Cloud",
          "Support Android & iOS",
          "Maksimal 1 Toko",
          "Maksimal 1 Kasir",
          "Manajemen Produk sederhana",
          "Laporan Penjualan Harian",
          "Export laporan ke Excel",
          "Update otomatis",
          "Support via Chat (Jam Kerja)",
        ],
        buttonText: "Pilih",
        href: "#",
        isPopular: false,
      },
      {
        name: "PROFESSIONAL",
        price: "5000000",
        tagline: "Lebih Lengkap, Lebih Profesional",
        title: "POS Professional",
        description:
          "Cocok untuk usaha berkembang dengan kebutuhan multi kasir dan laporan detail.",
        features: [
          "Aplikasi POS berbasis Cloud",
          "Support Android & iOS",
          "Maksimal 3 Toko",
          "Maksimal 5 Kasir",
          "Manajemen Produk & Stok",
          "Kategori Produk & Diskon",
          "Laporan Penjualan & Stok",
          "Integrasi Printer Bluetooth",
          "Export laporan (Excel & PDF)",
          "Support via Chat & Call (Jam Kerja)",
          "Proses setup cepat",
        ],
        buttonText: "Pilih",
        href: "#",
        isPopular: false,
      },
      {
        name: "BUSINESS",
        price: "10000000",
        tagline: "POS Handal untuk Bisnis Skala Menengah",
        title: "POS Business",
        description:
          "Cocok untuk bisnis profesional dan perusahaan menengah dengan kebutuhan lebih kompleks.",
        features: [
          "Aplikasi POS berbasis Cloud",
          "Support Android, iOS & Web Dashboard",
          "Maksimal 10 Toko",
          "Maksimal 20 Kasir",
          "Manajemen Produk, Stok & Supplier",
          "Multi Gudang & Transfer Stok",
          "Laporan Penjualan Lengkap",
          "Integrasi Printer Bluetooth & Thermal",
          "Integrasi QRIS (GoPay, OVO, Dana, ShopeePay, dll)",
          "Export laporan (Excel, PDF, CSV)",
          "Support Premium (Chat, Call, Zoom)",
          "Training Online 2x",
          "Proses setup 7-10 Hari",
        ],
        buttonText: "Pilih",
        href: "#",
        isPopular: true,
      },
      {
        name: "ENTERPRISE",
        price: "",
        tagline: "Custom POS Sesuai Kebutuhan Anda",
        title: "POS Enterprise",
        description:
          "Cocok untuk perusahaan besar, retail chain, restoran franchise, atau bisnis dengan integrasi khusus.",
        features: [
          "Full Custom POS Solution",
          "Support Android, iOS & Web Dashboard",
          "Unlimited Toko",
          "Unlimited Kasir",
          "Manajemen Produk, Stok, Supplier & Customer",
          "Multi Gudang, Transfer Stok, Forecasting",
          "Integrasi QRIS & Payment Gateway",
          "Integrasi Sistem Internal (ERP, Akuntansi, HR, dll)",
          "Laporan & Analitik Tingkat Lanjut",
          "Support On-site & SLA Khusus",
          "Training On-site / Online",
          "Proses Pengerjaan 1 - 3 Bulan (Sesuai Kesepakatan)",
          "Free Maintenance 30 Hari",
        ],
        buttonText: "Hubungi Kami",
        href: "#",
        isPopular: false,
      },
    ],
  },
  supportCta: {
    title: "Mau tahu SakaPOS lebih lanjut?",
    description: "Customer Service kami siap melayani anda kapan saja!",
    button: {
      label: "Ajukan Pertanyaan",
      href: "##",
    },
    image: "/phone.png",
  },
  finalCta: {
    title: "Ayo gunakan SakaPOS sekarang juga!",
    description: "Transaksi cepat, bisnis jadi lebih lancar!",
    button: {
      label: "Buat Akun",
      href: "/register",
    },
  },
  footer: {
    brand: "SakaPOS",
    brandHref: "/",
    address: "KH. Wahid Hasyim st. Gg. 01 No. 71 Kauman, Pekalongan, 51128",
    copyright: "All rights reserved by © Riyan.id 2024",
    navigationGroups: [
      [
        { label: "Home", href: "#" },
        { label: "Tentang Kami", href: "#" },
        { label: "Fitur", href: "#" },
        { label: "Pricing", href: "#" },
      ],
      [
        { label: "Blog", href: "#" },
        { label: "Kebijakan Privasi", href: "#" },
        { label: "Bantuan", href: "#" },
        { label: "Hubungi Kami", href: "#" },
      ],
      [
        { label: "Syarat & Ketentuan", href: "#" },
        { label: "Dashboard", href: "#" },
        { label: "Report", href: "#" },
        { label: "Hardware", href: "#" },
      ],
    ],
    email: "marketing@riyan.id",
    socialLinks: [
      {
        label: "Facebook",
        href: "https://www.facebook.com/riyanflashm/",
      },
      {
        label: "Twitter",
        href: "https://twitter.com/Muhamad_Riyan28",
      },
      {
        label: "LinkedIn",
        href: "https://www.linkedin.com/in/muhamad-riyan/",
      },
    ],
  },
} as const

export const organizationSchema = {
  "@context": "https://schema.org",
  "@type": "Organization",
  name: "SakaPOS",
  url: BASE_URL,
  logo: `${BASE_URL}/logo.png`,
  description:
    "Sistem Point of Sale (POS) digital untuk UMKM, Retail, dan Restoran dengan fitur manajemen stok otomatis, laporan penjualan instan, dan integrasi pembayaran digital.",
  address: {
    "@type": "PostalAddress",
    streetAddress: "KH. Wahid Hasyim st. Gg. 01 No. 71 Kauman",
    addressLocality: "Pekalongan",
    postalCode: "51128",
    addressCountry: "ID",
  },
  contactPoint: {
    "@type": "ContactPoint",
    telephone: "+62-xxx-xxxx-xxxx",
    contactType: "customer service",
    email: "marketing@riyan.id",
  },
  sameAs: [
    "https://www.facebook.com/pt.riyan.solusi.teknologi",
    "https://www.instagram.com/ptriyansolusiteknologi",
    "https://www.linkedin.com/company/riyan-id",
  ],
} as const

export const softwareSchema = {
  "@context": "https://schema.org",
  "@type": "SoftwareApplication",
  name: "SakaPOS",
  applicationCategory: "Point of Sale Software",
  operatingSystem: "Android, iOS, Web",
  offers: [
    {
      "@type": "Offer",
      name: "POS Basic",
      price: "3000000",
      priceCurrency: "IDR",
      description:
        "Cocok untuk UMKM atau usaha kecil yang baru mulai digitalisasi kasir.",
    },
    {
      "@type": "Offer",
      name: "POS Professional",
      price: "5000000",
      priceCurrency: "IDR",
      description:
        "Cocok untuk usaha berkembang dengan kebutuhan multi kasir dan laporan detail.",
    },
    {
      "@type": "Offer",
      name: "POS Business",
      price: "10000000",
      priceCurrency: "IDR",
      description:
        "Cocok untuk bisnis profesional dan perusahaan menengah dengan kebutuhan lebih kompleks.",
    },
  ],
  aggregateRating: {
    "@type": "AggregateRating",
    ratingValue: "4.8",
    ratingCount: "150",
    bestRating: "5",
    worstRating: "1",
  },
  features: [
    "Manajemen Stok Otomatis",
    "Laporan Penjualan Instan",
    "Multi-Outlet & Multi-User",
    "Integrasi Pembayaran Digital",
    "Cloud-Based & Aman",
    "Antarmuka Mudah & Intuitif",
  ],
} as const

export const faqSchema = {
  "@context": "https://schema.org",
  "@type": "FAQPage",
  mainEntity: landingData.faqs.faq_list.map((faq) => ({
    "@type": "Question",
    name: faq.question,
    acceptedAnswer: {
      "@type": "Answer",
      text: faq.answer,
    },
  })),
}

export const webPageSchema = {
  "@context": "https://schema.org",
  "@type": "WebPage",
  name: "SakaPOS - Kasir Digital untuk UMKM, Retail, dan Restoran",
  description:
    "Hemat waktu, kurangi kesalahan, dan tingkatkan keuntungan dengan SakaPOS. Sistem POS digital dengan fitur manajemen stok otomatis, laporan penjualan instan, dan integrasi pembayaran digital.",
  url: BASE_URL,
  mainEntity: {
    "@type": "Organization",
    name: "SakaPOS",
  },
  breadcrumb: {
    "@type": "BreadcrumbList",
    itemListElement: [
      {
        "@type": "ListItem",
        position: 1,
        name: "Home",
        item: BASE_URL,
      },
    ],
  },
} as const

export const seoMetadata = {
  title:
    "SakaPOS - Kasir Digital untuk UMKM, Retail & Restoran | Sistem POS Terbaik Indonesia",
  description:
    "Sistem Point of Sale (POS) digital terbaik untuk UMKM, retail, dan restoran. Fitur manajemen stok otomatis, laporan penjualan instan, integrasi QRIS & e-wallet. Tingkatkan efisiensi bisnis Anda dengan SakaPOS.",
  keywords:
    "sakapos, pos system, kasir digital, sistem kasir, point of sale, umkm, retail, restoran, manajemen stok, laporan penjualan, qris, e-wallet, gopay, ovo, dana, shopeepay",
  author: "SakaPOS",
  canonical: BASE_URL,
  themeColor: "#3B82F6",
  locale: "id_ID",
  siteName: "SakaPOS",
  url: BASE_URL,
  image: `${BASE_URL}/og-image.png`,
  imageAlt: "SakaPOS - Sistem Kasir Digital Terbaik",
  twitterHandle: "@Muhamad_Riyan28",
} as const

export const metaTags = [
  { name: "description", content: seoMetadata.description },
  { name: "keywords", content: seoMetadata.keywords },
  { name: "author", content: seoMetadata.author },
  { name: "robots", content: "index, follow" },
  { name: "viewport", content: "width=device-width, initial-scale=1.0" },
  { name: "theme-color", content: seoMetadata.themeColor },
  { name: "mobile-web-app-capable", content: "yes" },
  { name: "apple-mobile-web-app-capable", content: "yes" },
  { name: "apple-mobile-web-app-status-bar-style", content: "default" },
  { name: "apple-mobile-web-app-title", content: "SakaPOS" },
  { property: "og:type", content: "website" },
  { property: "og:title", content: seoMetadata.title },
  {
    property: "og:description",
    content:
      "tes desc Hemat waktu, kurangi kesalahan, dan tingkatkan keuntungan dengan SakaPOS. Sistem POS digital dengan fitur lengkap untuk semua jenis bisnis.",
  },
  { property: "og:url", content: seoMetadata.url },
  { property: "og:site_name", content: seoMetadata.siteName },
  { property: "og:image", content: seoMetadata.image },
  { property: "og:image:width", content: "1200" },
  { property: "og:image:height", content: "630" },
  { property: "og:image:alt", content: seoMetadata.imageAlt },
  { property: "og:locale", content: seoMetadata.locale },
  { name: "twitter:card", content: "summary_large_image" },
  { name: "twitter:title", content: seoMetadata.title },
  {
    name: "twitter:description",
    content:
      "Sistem POS digital dengan fitur manajemen stok otomatis, laporan penjualan instan, dan integrasi pembayaran digital.",
  },
  { name: "twitter:image", content: `${BASE_URL}/twitter-image.png` },
  { name: "twitter:image:alt", content: "SakaPOS - Sistem Kasir Digital" },
  { name: "twitter:site", content: seoMetadata.twitterHandle },
  { name: "twitter:creator", content: seoMetadata.twitterHandle },
  { name: "geo.region", content: "ID-33" },
  { name: "geo.placename", content: "Pekalongan" },
  { name: "geo.position", content: "-6.8886;109.6753" },
  { name: "ICBM", content: "-6.8886, 109.6753" },
  {
    name: "business:contact_data:street_address",
    content: "KH. Wahid Hasyim st. Gg. 01 No. 71 Kauman",
  },
  { name: "business:contact_data:locality", content: "Pekalongan" },
  { name: "business:contact_data:postal_code", content: "51128" },
  { name: "business:contact_data:country_name", content: "Indonesia" },
]

export const linkTags = [
  { rel: "canonical", href: seoMetadata.canonical },
  { rel: "icon", type: "image/x-icon", href: "/favicon.ico" },
  {
    rel: "apple-touch-icon",
    sizes: "180x180",
    href: "/apple-touch-icon.png",
  },
  {
    rel: "icon",
    type: "image/png",
    sizes: "32x32",
    href: "/favicon-32x32.png",
  },
  {
    rel: "icon",
    type: "image/png",
    sizes: "16x16",
    href: "/favicon-16x16.png",
  },
]
