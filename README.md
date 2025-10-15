# Invoice Generator

[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blue)](https://tauri.app/)
[![Svelte 5](https://img.shields.io/badge/Svelte-5.0-orange)](https://svelte.dev/)
[![Bun](https://img.shields.io/badge/Bun-ts-pink)](https://bun.sh/)
[![shadcn-svelte](https://img.shields.io/badge/UI-shadcn--svelte-purple)](https://www.shadcn-svelte.com/)

A professional, cross-platform desktop application for generating PDF invoices. Built with Tauri 2, Svelte 5, and Rust for optimal performance and a beautiful, modern UI.

## ✨ Features

### Invoice Generation
- 📄 **Professional PDF Output** - Generate clean, professional invoices with customizable layouts
- 🔢 **Sequential Numbering** - Automatic invoice numbering with customizable templates (e.g., `INV_{sequence}`)
- 💰 **Service Billing** - Configure service descriptions, hourly rates, and hours worked
- 🧾 **Itemized Breakdown** - Clear display of services, quantities, rates, and totals

### Settings & Persistence
- 🏦 **Bank Account Details** - Store beneficiary account information for wire transfers
- 📍 **Address Management** - Save beneficiary address with support for multi-line addresses
- 💾 **Client Data Persistence** - Automatically saves last used client information
- 🎨 **Theme Support** - Light and dark mode options

### User Experience
- 🚀 **Fast & Responsive** - Built with Rust and Svelte for optimal performance
- 📱 **Modern UI** - Beautiful interface with shadcn-svelte components
- 💅 **TailwindCSS** - Professionally styled with utility-first CSS
- 🔧 **Type-Safe** - Full TypeScript support for reliability
- 🖥️ **Cross-Platform** - Works on Windows, macOS, and Linux

## 🚀 Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

1. **Bun** - [Bun installation](https://bun.sh/docs/installation)
2. **Rust** - [Rust installation](https://www.rust-lang.org/tools/install)
3. **For Windows Users:**
   - Install [Visual Studio Community](https://visualstudio.microsoft.com/vs/community/)
   - During installation, select "Desktop development with C++" workload

### Installation

1. Clone the repository:
```bash
git clone https://github.com/zehurzeda/tauri-invoice-generator.git
```

2. Navigate to the project directory:
```bash
cd tauri-invoice-generator
```

3. Install dependencies:
```bash
bun install
```

4. Run the application:
```bash
bun run tauri dev
```

## 📖 How to Use

### Initial Setup

1. **Configure Bank Details** (Settings > Bank Data)
   - Enter beneficiary account name
   - Add bank name and address
   - Provide account type, number, wire routing, and SWIFT code

2. **Configure Address** (Settings > Address)
   - Enter your business address (Line 1 and optional Line 2)
   - Select state and enter city
   - Provide ZIP code

### Generating an Invoice

1. The app opens directly to the **Generate** page
2. Fill in the client information:
   - Client name
   - Client address (Line 1 and optional Line 2)
   - Client email (optional)
3. Configure service details:
   - Service description (e.g., "Web Development", "Consulting")
   - Hourly rate
   - Hours worked
   - View the automatically calculated total
4. Set invoice options:
   - Filename template (use `{sequence}` for auto-numbering)
   - Optional notes
5. Click **"Gerar Invoice em PDF"**
6. Choose where to save the PDF
7. The invoice opens automatically after generation

### Features in Detail

- **Sequential Numbering**: Invoices are automatically numbered (001, 002, etc.)
- **Template Support**: Use `INV_{sequence}` to create filenames like `INV_001.pdf`
- **Data Persistence**: Client information is saved and pre-filled for next time
- **Professional Layout**: Clean, business-ready PDF format with all necessary details

## 🛠️ Development

### Start Development Server
```bash
bun run tauri dev
```

### Build Production Release
```bash
bun run tauri build
```

The built application will be available in `src-tauri/target/release/bundle/`

### Code Formatting
```bash
bun run format
```

## 🏗️ Tech Stack

### Frontend
- **Svelte 5** - Reactive UI framework with runes
- **SvelteKit** - Application framework with routing
- **TypeScript** - Type safety and better DX
- **TailwindCSS** - Utility-first styling
- **shadcn-svelte** - Accessible UI components
- **Zod** - Schema validation
- **Superforms** - Form handling with validation

### Backend
- **Rust** - High-performance backend logic
- **Tauri 2** - Desktop application framework
- **printpdf** - PDF generation library
- **serde** - Serialization/deserialization

### Build Tools
- **Bun** - Fast JavaScript runtime and package manager
- **Cargo** - Rust package manager

## 📚 Documentation Links

- [Svelte 5 Documentation](https://svelte.dev/)
- [Tauri 2 Documentation](https://tauri.app/start/)
- [TailwindCSS Documentation](https://tailwindcss.com/)
- [shadcn-svelte Components](https://next.shadcn-svelte.com/)
- [printpdf Crate](https://docs.rs/printpdf/)

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch
3. Submit a Pull Request

Please ensure your PR follows the project's coding standards.

## 📝 License

This project is open source and available under the MIT License.

## ⚠️ Platform Support

Tested and verified on:
- ✅ macOS (Apple Silicon & Intel)
- ✅ Windows 11
- ✅ Linux (Ubuntu/Debian-based distributions)

## 🐛 Known Issues

None currently. Please report any issues you encounter!

## 💡 Future Enhancements

Potential features for future releases:
- Multiple currency support
- Tax calculation options
- Invoice templates selection
- Export to different formats (CSV, JSON)
- Multiple language support
- Client database management
- Invoice history and search