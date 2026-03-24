import { Accordion } from "@/components/custom/accordion"
import { OrbitingCircles } from "@/components/custom/orbiting-circles"
import Pricing from "@/components/custom/pricing"
import { StarRating } from "@/components/custom/star-rating"
import WordRotate from "@/components/custom/word-rotate"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { BASE_URL } from "@/constants"
import { motionProps } from "@/lib/reveal"
import { DynamicIcon } from "lucide-react/dynamic"
import { motion } from "motion/react"
import { Helmet } from "react-helmet-async"
import { IoMdMail } from "react-icons/io"
import { Link } from "react-router"

export default function LandingPage() {
  const landingData = {
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
  }

  // JSON-LD structured data
  const organizationSchema = {
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
  }

  const softwareSchema = {
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
  }

  const faqSchema = {
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

  const webPageSchema = {
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
  }

  // SEO Metadata Configuration
  const seoMetadata = {
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
  }

  const metaTags = [
    // Basic Meta Tags
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

    // Open Graph Tags
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

    // Twitter Card Tags
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

    // Geo Tags
    { name: "geo.region", content: "ID-33" },
    { name: "geo.placename", content: "Pekalongan" },
    { name: "geo.position", content: "-6.8886;109.6753" },
    { name: "ICBM", content: "-6.8886, 109.6753" },

    // Business Schema
    {
      name: "business:contact_data:street_address",
      content: "KH. Wahid Hasyim st. Gg. 01 No. 71 Kauman",
    },
    { name: "business:contact_data:locality", content: "Pekalongan" },
    { name: "business:contact_data:postal_code", content: "51128" },
    { name: "business:contact_data:country_name", content: "Indonesia" },
  ]

  const linkTags = [
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

  return (
    <>
      <Helmet>
        <title>{seoMetadata.title}</title>
      </Helmet>
      {metaTags.map((tag, index) => (
        <meta key={index} {...tag} />
      ))}
      {linkTags.map((link, index) => (
        <link key={index} {...link} />
      ))}
      <script type="application/ld+json">
        {JSON.stringify(organizationSchema)}
      </script>
      <script type="application/ld+json">
        {JSON.stringify(softwareSchema)}
      </script>
      <script type="application/ld+json">{JSON.stringify(faqSchema)}</script>
      <script type="application/ld+json">
        {JSON.stringify(webPageSchema)}
      </script>

      <main className="flex flex-col items-stretch justify-center overflow-hidden">
        <section
          id="home"
          className="bg-primary relative overflow-hidden pt-32 md:pt-40"
        >
          <div className="container mx-auto px-3 sm:max-w-[540px] md:max-w-[720px] md:px-20 lg:max-w-[1140px] xl:max-w-[1690px]">
            <div className="mb-10 grid grid-cols-1 gap-8 xl:grid-cols-2">
              <div
                className="relative mx-auto space-y-8 text-center sm:max-w-[570px] md:space-y-10 xl:mx-0 xl:text-start"
                data-cue="slideInUp"
                data-show="true"
                style={{
                  animation: "slideInUp 600ms ease 0ms normal forwards",
                }}
              >
                <div className="text-primary-foreground">
                  <motion.h1
                    className="mb-5 text-2xl font-bold text-balance sm:text-3xl md:mb-7 md:text-4xl lg:mb-5 lg:text-5xl lg:leading-[1.2] xl:mb-6"
                    {...motionProps({ reveal: "bottom" })}
                  >
                    <span>SakaPOS — Kasir Digital untuk</span>
                    <WordRotate
                      words={["UMKM", "Retail", "Resto", "mu!"]}
                      className="pl-2 md:pl-3"
                    />
                  </motion.h1>
                  <motion.p
                    className="text-sm md:lg:text-base xl:text-lg"
                    {...motionProps({ reveal: "bottom", delay: 0.3 })}
                  >
                    Hemat waktu, kurangi kesalahan, dan tingkatkan keuntungan
                    dengan SakaPOS.
                  </motion.p>
                </div>
                <motion.div
                  className="flex items-center justify-center space-x-4 xl:justify-start"
                  {...motionProps({ reveal: "bottom", delay: 0.6 })}
                >
                  <Button
                    variant="white"
                    href="/register"
                    className="font-semibold"
                  >
                    Buat Akun
                  </Button>
                  <Button variant="success" className="font-semibold">
                    Konsultasi Dulu
                  </Button>
                </motion.div>
                <motion.div
                  className="flex items-center justify-center space-x-[15px] xl:justify-start ltr:ml-[2px] rtl:mr-[2px]"
                  {...motionProps({ reveal: "left", delay: 0.9 })}
                >
                  <div className="flex -space-x-2">
                    <img
                      src="/user-1.webp"
                      className="inline-block size-12 rounded-full object-cover object-top ring-[3px] ring-white"
                      alt="image"
                    />
                    <img
                      src="/user-1.webp"
                      className="inline-block size-12 rounded-full object-cover object-top ring-[3px] ring-white"
                      alt="image"
                    />
                    <img
                      src="/user-1.webp"
                      className="inline-block size-12 rounded-full object-cover object-top ring-[3px] ring-white"
                      alt="image"
                    />
                  </div>
                  <div>
                    <StarRating showRating={false} delay={0.9} />
                    <p className="text-primary-foreground">Review 4.8/5.0</p>
                  </div>
                </motion.div>
              </div>
              <motion.div
                className="text-center xl:text-end"
                data-cue="slideInUp"
                data-show="true"
                {...motionProps({ reveal: "right", delay: 0.9 })}
              >
                <img
                  src="/hero-device.png"
                  className="inline-block"
                  alt="banner5"
                />
              </motion.div>
            </div>
          </div>
          <div className="bg-background overflow-hidden">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 1440 320"
              className="text-primary"
            >
              <path
                fill="currentColor"
                fillOpacity="1"
                d="M0,224L40,208C80,192,160,160,240,160C320,160,400,192,480,181.3C560,171,640,117,720,128C800,139,880,213,960,240C1040,267,1120,245,1200,224C1280,203,1360,181,1400,170.7L1440,160L1440,0L1400,0C1360,0,1280,0,1200,0C1120,0,1040,0,960,0C880,0,800,0,720,0C640,0,560,0,480,0C400,0,320,0,240,0C160,0,80,0,40,0L0,0Z"
              ></path>
            </svg>
          </div>
        </section>

        <section
          id="features"
          className="space-y-8 px-10 py-20 md:space-y-16 md:py-40 lg:px-16"
        >
          <div className="flex flex-col items-center gap-4 text-center">
            <motion.div {...motionProps({ reveal: "bottom" })}>
              <Badge className="rounded-full px-4">
                {landingData.features.tagline}
              </Badge>
            </motion.div>
            <motion.h1
              className="text-2xl font-semibold md:text-4xl"
              {...motionProps({ reveal: "bottom", delay: 0.3 })}
            >
              <span>{landingData.features.title[1]}</span>
              <span className="text-primary underline decoration-wavy md:decoration-4">
                {landingData.features.title[2]}
              </span>
              <span>{landingData.features.title[3]}</span>
            </motion.h1>
            <motion.p
              className="max-w-3xl text-sm text-balance md:text-base"
              {...motionProps({ reveal: "bottom", delay: 0.6 })}
            >
              {landingData.features.description}
            </motion.p>
          </div>
          <div className="flex flex-col items-center gap-4 md:flex-row">
            {/* <div className="flex-none w-full md:w-1/2 flex justify-center">
              <img src="/laptop-phone.png" alt="" className="object-contain" />
            </div> */}
            <div className="grid grid-cols-1 gap-6 md:grid-cols-2 2xl:grid-cols-3">
              {landingData.features.feature_list.map((item, index) => (
                <motion.div
                  key={index}
                  className="flex items-center gap-8 rounded-xl border p-4 shadow-md"
                  {...motionProps({
                    reveal: "zoomIn",
                    delay: index * 0.3 + 1.2,
                  })}
                >
                  <div className="bg-primary rounded-full p-4">
                    <DynamicIcon
                      // eslint-disable-next-line @typescript-eslint/no-explicit-any
                      name={item.icon as any}
                      size={32}
                      strokeWidth={1.6}
                      className="text-primary-foreground"
                    />
                  </div>
                  <div className="flex flex-col gap-1">
                    <h4 className="text-lg font-semibold md:text-xl">
                      {item.title}
                    </h4>
                    <span>{item.description}</span>
                  </div>
                </motion.div>
              ))}
            </div>
          </div>
        </section>

        <section id="benefits">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 1440 320"
            className="text-sky-100"
          >
            <path
              fill="currentColor"
              fillOpacity="1"
              d="M0,192L40,170.7C80,149,160,107,240,96C320,85,400,107,480,138.7C560,171,640,213,720,202.7C800,192,880,128,960,117.3C1040,107,1120,149,1200,176C1280,203,1360,213,1400,218.7L1440,224L1440,320L1400,320C1360,320,1280,320,1200,320C1120,320,1040,320,960,320C880,320,800,320,720,320C640,320,560,320,480,320C400,320,320,320,240,320C160,320,80,320,40,320L0,320Z"
            ></path>
          </svg>
          <div className="flex w-full flex-col bg-sky-100">
            <div className="flex flex-col items-center gap-4 text-center">
              <motion.div {...motionProps({ reveal: "bottom" })}>
                <Badge className="rounded-full px-4">
                  {landingData.benefits.tagline}
                </Badge>
              </motion.div>
              <motion.h1
                className="text-2xl font-semibold md:text-4xl"
                {...motionProps({ reveal: "bottom", delay: 0.3 })}
              >
                {landingData.benefits.title}
              </motion.h1>
              <motion.p
                className="max-w-3xl text-sm text-balance md:text-base"
                {...motionProps({ reveal: "bottom", delay: 0.6 })}
              >
                {landingData.benefits.description}
              </motion.p>
            </div>
            {landingData.benefits.benefit_list.map((item, index) => (
              <div
                key={index}
                className="flex flex-col-reverse items-center gap-10 px-10 py-20 md:px-20 lg:px-40 odd:lg:flex-row even:lg:flex-row-reverse"
              >
                <div className="w-full space-y-8 lg:w-1/2">
                  <div className="space-y-1">
                    <motion.h2
                      className="text-xl font-bold sm:text-4xl"
                      {...motionProps({ reveal: "bottom" })}
                    >
                      <span>{item.title}</span>
                      {/* <span className="inline-block bg-gradient-to-br from-primary to-sky-300 bg-clip-text text-transparent">
                        Kontrol Penuh
                      </span>
                      <span>{` atas Penjualan & Stok`}</span> */}
                    </motion.h2>
                    <motion.p
                      {...motionProps({ reveal: "bottom", delay: 0.3 })}
                    >
                      {item.description}
                    </motion.p>
                  </div>
                  <motion.ul
                    className="space-y-3"
                    {...motionProps({ reveal: "bottom", delay: 0.6 })}
                  >
                    {item.points.map((point, index) => (
                      <motion.li
                        key={index}
                        className="flex gap-4"
                        {...motionProps({
                          reveal: "bottom",
                          delay: index * 0.3 + 0.9,
                        })}
                      >
                        <div className="bg-primary size-10 rounded-xl p-2">
                          <DynamicIcon
                            // eslint-disable-next-line @typescript-eslint/no-explicit-any
                            name={point.icon as any}
                            className="text-primary-foreground flex-none text-2xl"
                          />
                        </div>
                        <span>{point.text}</span>
                      </motion.li>
                    ))}
                  </motion.ul>
                </div>
                <motion.img
                  src={item.image}
                  alt="supermarket-workers"
                  className="w-full object-contain lg:w-1/2"
                  {...motionProps({
                    reveal: "zoomIn",
                    viewportMargin: "0px 0px -500px 0px",
                  })}
                />
              </div>
            ))}
          </div>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 1440 320"
            className="text-sky-100"
          >
            <path
              fill="currentColor"
              fillOpacity="1"
              d="M0,32L48,53.3C96,75,192,117,288,128C384,139,480,117,576,101.3C672,85,768,75,864,90.7C960,107,1056,149,1152,154.7C1248,160,1344,128,1392,112L1440,96L1440,0L1392,0C1344,0,1248,0,1152,0C1056,0,960,0,864,0C768,0,672,0,576,0C480,0,384,0,288,0C192,0,96,0,48,0L0,0Z"
            ></path>
          </svg>
        </section>

        <section
          id="payment-integration"
          className="flex flex-col items-center gap-16 py-20"
        >
          <div className="flex max-w-4xl flex-col items-center gap-4 px-4 text-center">
            <motion.div {...motionProps({ reveal: "bottom" })}>
              <Badge className="rounded-full px-4">
                {landingData.integrations.tagline}
              </Badge>
            </motion.div>
            <motion.h1
              className="text-2xl font-semibold md:text-4xl"
              {...motionProps({ reveal: "bottom", delay: 0.3 })}
            >
              {landingData.integrations.title}
            </motion.h1>
            <motion.p {...motionProps({ reveal: "bottom", delay: 0.6 })}>
              {landingData.integrations.description}
            </motion.p>
          </div>
          <motion.div
            className="relative flex h-[300px] w-full items-end overflow-hidden"
            {...motionProps({ reveal: "bottom", delay: 0.9 })}
          >
            <div className="relative flex h-[700px] w-full flex-col items-center justify-center overflow-hidden">
              <OrbitingCircles iconSize={52} radius={300}>
                <img
                  src="/svg/payment/dana.svg"
                  alt="dana"
                  className="h-full w-full object-contain"
                />
                <img
                  src="/svg/payment/gopay.svg"
                  alt="gopay"
                  className="h-full w-full object-contain"
                />
                <img
                  src="/svg/payment/shopeepay.svg"
                  alt="shopeepay"
                  className="h-full w-full object-contain"
                />
                <img
                  src="/svg/payment/qris.svg"
                  alt="qris"
                  className="h-full w-full object-contain"
                />
              </OrbitingCircles>
              <OrbitingCircles iconSize={52} radius={200} reverse>
                <img
                  src="/svg/payment/ovo.svg"
                  alt="ovo"
                  className="h-full w-full object-contain"
                />
                <img
                  src="/svg/payment/linkaja.svg"
                  alt="linkaja"
                  className="h-full w-full object-contain"
                />
                <img
                  src="/svg/payment/doku.svg"
                  alt="doku"
                  className="h-full w-full object-contain"
                />
              </OrbitingCircles>
            </div>
            <div className="ground absolute inset-x-0 top-0 h-1/2 bg-gradient-to-b from-white via-white/0 to-white/0" />
          </motion.div>
        </section>

        <section
          id="pricing"
          className="flex flex-col items-center gap-20 px-20 py-20 md:py-40"
        >
          <div className="flex max-w-4xl flex-col items-center gap-4 px-4 text-center">
            <motion.div {...motionProps({ reveal: "bottom" })}>
              <Badge className="rounded-full px-4">
                {landingData.pricing.tagline}
              </Badge>
            </motion.div>
            <motion.h1
              className="text-2xl font-semibold md:text-4xl"
              {...motionProps({ reveal: "bottom", delay: 0.3 })}
            >
              {landingData.pricing.title}
            </motion.h1>
            <motion.p {...motionProps({ reveal: "bottom", delay: 0.6 })}>
              {landingData.pricing.description}
            </motion.p>
          </div>
          <motion.div {...motionProps({ reveal: "bottom", delay: 0.9 })}>
            <Pricing plans={landingData.pricing.price_list} />
          </motion.div>
        </section>

        <section
          id="faq"
          className="flex flex-col items-center gap-20 px-4 py-20 md:px-20 md:py-40"
        >
          <div className="flex flex-col items-center gap-4 text-center">
            <motion.div {...motionProps({ reveal: "bottom" })}>
              <Badge className="rounded-full px-4">
                {landingData.faqs.tagline}
              </Badge>
            </motion.div>
            <motion.h1
              className="text-2xl font-semibold md:text-4xl"
              {...motionProps({ reveal: "bottom", delay: 0.3 })}
            >
              {landingData.faqs.title}
            </motion.h1>
          </div>
          <div className="w-full max-w-3xl space-y-2 px-4 md:space-y-4">
            {landingData.faqs.faq_list.map((item, index) => (
              <Accordion
                key={index}
                index={index}
                question={item.question}
                answer={item.answer}
              />
            ))}
          </div>
        </section>

        {/* <section
          id="blog"
          className="flex w-full flex-col gap-10 px-10 py-20 md:px-20"
        >
          <h1 className="text-center text-2xl font-bold md:text-3xl">
            Mungkin Kamu Juga Membutuhkan Ini
          </h1>
          <div className="flex w-full">
            <Swiper
              rewind={true}
              grabCursor={true}
              slidesPerView={1}
              slidesPerGroup={1}
              autoplay={{
                delay: 2500,
                disableOnInteraction: false,
              }}
              breakpoints={{
                768: {
                  slidesPerView: 3,
                  slidesPerGroup: 3,
                },
              }}
              navigation={{
                enabled: true,
              }}
              modules={[EffectCoverflow, Navigation, Autoplay]}
              className="swiper_container"
            >
              {blogs.map((data) => (
                <SwiperSlide key={data.id}>
                  <div className="relative flex w-full justify-center">
                    <Link className="flex w-[95%] flex-col gap-2" href="##">
                      <div className="flex flex-col gap-1">
                        <span className="font-light">{data.updatedAt}</span>
                        <div className="relative aspect-[3/2] w-full rounded-xl overflow-hidden bg-green-300">
                          <img
                            src={data.thumb}
                            alt="slide_image"
                            fill
                            sizes="(max-width: 768px) 33vw, (max-width: 1200px) 70vw, 100vw"
                            className="object-cover w-full"
                          />
                        </div>
                      </div>
                      <div className="flex flex-col gap-1">
                        <p className="line-clamp-2 md:text-lg font-bold leading-snug text-slate-900">
                          {data.title}
                        </p>
                        <p className="line-clamp-4 leading-tight">
                          {data.content}
                        </p>
                      </div>
                    </Link>
                  </div>
                </SwiperSlide>
              ))}
            </Swiper>
          </div>
        </section> */}
        <section
          id="bantuan"
          className="relative w-full px-0 py-[1px] text-white sm:my-20 sm:py-10 md:px-10 md:py-20 lg:px-32"
        >
          <motion.div
            className="absolute top-0 left-10 hidden aspect-[9/16] h-full sm:block md:left-16 lg:left-56"
            {...motionProps({ reveal: "left" })}
          >
            <img src="/phone.png" alt="" className="object-contain" />
          </motion.div>
          <div className="bg-primary sm:bg-primary flex min-h-56 w-full flex-col items-end justify-center gap-10 px-10 py-10 pr-10 transition-all sm:items-start sm:pl-56 md:min-h-80 md:rounded-3xl md:pl-80 lg:pr-20 lg:pl-[420px]">
            <div className="flex flex-col items-end gap-1 sm:items-start">
              <motion.span
                className="text-xl font-semibold md:text-2xl lg:text-3xl"
                {...motionProps({ reveal: "bottom", delay: 0.3 })}
              >
                Mau tahu SakaPOS lebih lanjut?
              </motion.span>
              <motion.span
                className="font-light lg:text-lg"
                {...motionProps({ reveal: "bottom", delay: 0.6 })}
              >
                Customer Service kami siap melayani anda kapan saja!
              </motion.span>
            </div>
            <motion.div {...motionProps({ reveal: "bottom", delay: 0.9 })}>
              <Button
                href="##"
                variant="success"
                className="px-10 font-semibold"
              >
                Ajukan Pertanyaan
              </Button>
            </motion.div>
          </div>
        </section>

        <section>
          <div className="item-start bg-primary flex w-full flex-col gap-5 px-10 py-10 text-white md:flex-row md:items-center md:justify-between md:gap-10 md:px-20 md:pb-0 lg:translate-y-10">
            <div className="flex flex-col gap-3">
              <motion.span
                className="text-xl font-semibold md:text-2xl lg:text-3xl"
                {...motionProps({ reveal: "bottom" })}
              >
                Ayo gunakan SakaPOS sekarang juga!
              </motion.span>
              <motion.span
                className="font-light lg:text-xl"
                {...motionProps({ reveal: "bottom", delay: 0.3 })}
              >
                Transaksi cepat, bisnis jadi lebih lancar!
              </motion.span>
            </div>
            <motion.div
              className="flex items-center"
              {...motionProps({ reveal: "right", delay: 0.6 })}
            >
              <Button
                href="/register"
                variant="success"
                className="h-9 px-10 font-semibold"
              >
                Buat Akun
              </Button>
            </motion.div>
          </div>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 1440 320"
            className="text-primary"
          >
            <path
              fill="currentColor"
              fillOpacity="1"
              d="M0,128L120,117.3C240,107,480,85,720,90.7C960,96,1200,128,1320,144L1440,160L1440,0L1320,0C1200,0,960,0,720,0C480,0,240,0,120,0L0,0Z"
            ></path>
          </svg>
        </section>

        <footer className="flex w-full flex-col gap-10 p-10 pt-0 lg:-translate-y-28 lg:p-20 lg:py-0">
          <motion.a
            href="/"
            className="text-primary inline-flex flex-none items-center text-xl font-extrabold transition-colors duration-700"
            {...motionProps({ reveal: "bottom" })}
          >
            SakaPOS
          </motion.a>
          <div className="flex w-full flex-col justify-between gap-5 lg:flex-row lg:gap-20">
            <div className="flex flex-col-reverse gap-10 lg:flex-row lg:gap-20">
              <div className="flex max-w-sm min-w-80 flex-col gap-4">
                <motion.span {...motionProps({ reveal: "bottom", delay: 0.3 })}>
                  KH. Wahid Hasyim st. Gg. 01 No. 71 Kauman, Pekalongan, 51128
                </motion.span>
                <motion.span {...motionProps({ reveal: "bottom", delay: 0.6 })}>
                  All rights reserved by © Riyan.id 2024
                </motion.span>
              </div>
              <div className="flex w-full justify-between gap-4">
                <motion.div
                  className="flex flex-col gap-4"
                  {...motionProps({ reveal: "bottom", delay: 0.9 })}
                >
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Home
                  </Link>
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Tentang Kami
                  </Link>
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Fitur
                  </Link>
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Pricing
                  </Link>
                </motion.div>
                <motion.div
                  className="flex flex-col gap-4"
                  {...motionProps({ reveal: "bottom", delay: 1.2 })}
                >
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Blog
                  </Link>
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Kebijakan Privasi
                  </Link>
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Bantuan
                  </Link>
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Hubungi Kami
                  </Link>
                </motion.div>
                <motion.div
                  className="flex flex-col gap-4"
                  {...motionProps({ reveal: "bottom", delay: 1.5 })}
                >
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Syarat & Ketentuan
                  </Link>
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Dashboard
                  </Link>
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Report
                  </Link>
                  <Link className="hover:text-primary cursor-pointer" to="#">
                    Hardware
                  </Link>
                </motion.div>
              </div>
            </div>
            <div className="flex flex-none flex-col gap-2">
              <motion.div
                className="group flex items-center gap-2 hover:cursor-pointer"
                {...motionProps({ reveal: "bottom", delay: 1.8 })}
              >
                <IoMdMail className="transition-color group-hover:text-primary text-3xl duration-300 group-hover:drop-shadow-md" />
                <span>marketing@riyan.id</span>
              </motion.div>
              <motion.div
                className="flex gap-3"
                {...motionProps({ reveal: "bottom", delay: 2.1 })}
              >
                <a href="https://www.facebook.com/riyanflashm/">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="32"
                    height="33"
                    viewBox="0 0 32 33"
                    fill="none"
                    className="transition-color hover:text-primary duration-300 hover:drop-shadow-md"
                  >
                    <g clipPath="url(#clip0_162_129)">
                      <path
                        d="M32 16.4443C32 7.60684 24.8375 0.444336 16 0.444336C7.1625 0.444336 0 7.60684 0 16.4443C0 25.2818 7.1625 32.4443 16 32.4443C16.0938 32.4443 16.1875 32.4443 16.2812 32.438V19.9881H12.8438V15.9818H16.2812V13.0318C16.2812 9.61309 18.3688 7.75059 21.4188 7.75059C22.8813 7.75059 24.1375 7.85684 24.5 7.90684V11.4818H22.4C20.7437 11.4818 20.4188 12.2693 20.4188 13.4255V15.9755H24.3875L23.8687 19.9818H20.4188V31.8256C27.1063 29.9068 32 23.7505 32 16.4443Z"
                        fill="currentColor"
                      />
                    </g>
                    <defs>
                      <clipPath id="clip0_162_129">
                        <rect
                          width="32"
                          height="32"
                          fill="white"
                          transform="translate(0 0.444336)"
                        />
                      </clipPath>
                    </defs>
                  </svg>
                </a>
                <a href="https://twitter.com/Muhamad_Riyan28">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="32"
                    height="33"
                    viewBox="0 0 32 33"
                    fill="none"
                    className="transition-color hover:text-primary cursor-pointer duration-300 hover:drop-shadow-md"
                  >
                    <g clipPath="url(#clip0_162_133)">
                      <path
                        d="M16 0.444336C7.16479 0.444336 0 7.60913 0 16.4443C0 25.2795 7.16479 32.4443 16 32.4443C24.8352 32.4443 32 25.2795 32 16.4443C32 7.60913 24.8352 0.444336 16 0.444336ZM23.3054 12.9194C23.3125 13.0769 23.3159 13.2351 23.3159 13.394C23.3159 18.2468 19.6221 23.8427 12.8669 23.843C10.793 23.843 8.86304 23.2351 7.23779 22.1933C7.52515 22.2273 7.81763 22.2441 8.11377 22.2441C9.83447 22.2441 11.418 21.6572 12.675 20.6721C11.0674 20.6423 9.71191 19.5805 9.24414 18.1213C9.46802 18.1643 9.69824 18.1877 9.93433 18.1877C10.2695 18.1877 10.5942 18.1425 10.9028 18.0583C9.22241 17.7219 7.95654 16.2368 7.95654 14.4585C7.95654 14.4419 7.95654 14.427 7.95703 14.4116C8.4519 14.6867 9.01782 14.8523 9.62036 14.8708C8.63428 14.2129 7.98608 13.0881 7.98608 11.8139C7.98608 11.1411 8.16797 10.5107 8.4834 9.96802C10.2944 12.1901 13.001 13.6516 16.0532 13.8051C15.9902 13.5361 15.9578 13.2558 15.9578 12.9677C15.9578 10.9404 17.6025 9.29566 19.6306 9.29566C20.687 9.29566 21.6411 9.74219 22.3113 10.456C23.1479 10.291 23.9336 9.98536 24.6433 9.5647C24.3687 10.4219 23.7866 11.1411 23.0283 11.5959C23.7712 11.507 24.4792 11.31 25.1372 11.0175C24.6458 11.7541 24.0225 12.4011 23.3054 12.9194Z"
                        fill="currentColor"
                      />
                    </g>
                    <defs>
                      <clipPath id="clip0_162_133">
                        <rect
                          width="32"
                          height="32"
                          fill="currentColor"
                          transform="translate(0 0.444336)"
                        />
                      </clipPath>
                    </defs>
                  </svg>
                </a>
                <a href="https://www.linkedin.com/in/muhamad-riyan/">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="32"
                    height="33"
                    viewBox="0 0 32 33"
                    fill="none"
                    className="transition-color hover:text-primary cursor-pointer duration-300 hover:drop-shadow-md"
                  >
                    <g clipPath="url(#clip0_162_135)">
                      <path
                        d="M16 0.444336C7.16479 0.444336 0 7.60913 0 16.4443C0 25.2795 7.16479 32.4443 16 32.4443C24.8352 32.4443 32 25.2795 32 16.4443C32 7.60913 24.8352 0.444336 16 0.444336ZM11.3506 24.6318H7.45386V12.9084H11.3506V24.6318ZM9.40234 11.3076H9.37695C8.06934 11.3076 7.22363 10.4075 7.22363 9.28247C7.22363 8.13208 8.09521 7.25684 9.42822 7.25684C10.7612 7.25684 11.5815 8.13208 11.6069 9.28247C11.6069 10.4075 10.7612 11.3076 9.40234 11.3076ZM25.4014 24.6318H21.5051V18.3601C21.5051 16.7839 20.9409 15.7089 19.531 15.7089C18.4546 15.7089 17.8135 16.434 17.5317 17.134C17.4287 17.3845 17.4036 17.7346 17.4036 18.0849V24.6318H13.5071C13.5071 24.6318 13.5581 14.0083 13.5071 12.9084H17.4036V14.5683C17.9214 13.7695 18.8479 12.6333 20.9153 12.6333C23.479 12.6333 25.4014 14.3088 25.4014 17.9096V24.6318Z"
                        fill="currentColor"
                      />
                    </g>
                    <defs>
                      <clipPath id="clip0_162_135">
                        <rect
                          width="32"
                          height="32"
                          fill="currentColor"
                          transform="translate(0 0.444336)"
                        />
                      </clipPath>
                    </defs>
                  </svg>
                </a>
              </motion.div>
            </div>
          </div>
        </footer>
      </main>
    </>
  )
}
