/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'media',
  theme: {
    extend: {
      // 响应式断点 - 专注于桌面和平板
      screens: {
        'tablet': '768px',    // 平板设备
        'desktop': '1024px',  // 桌面设备
        'large': '1280px',    // 大桌面
        'xlarge': '1536px',   // 超大桌面
        '2xl': '1920px',      // 超宽屏
      },
      
      // 现代iOS App风格颜色系统
      colors: {
        primary: {
          50: '#f0f4ff',
          100: '#e0e9ff',
          200: '#c7d6fe',
          300: '#a5b8fc',
          400: '#818cf8',
          500: '#667eea',  // 主色调
          600: '#5a67d8',
          700: '#4c51bf',
          800: '#434190',
          900: '#3c366b',
          950: '#252244',
        },
        secondary: {
          50: '#fdf4ff',
          100: '#fae8ff',
          200: '#f5d0fe',
          300: '#f0abfc',
          400: '#e879f9',
          500: '#764ba2',  // 次要色
          600: '#c026d3',
          700: '#a21caf',
          800: '#86198f',
          900: '#701a75',
          950: '#4a044e',
        },
        // 渐变色配置
        gradient: {
          'blue-start': '#4facfe',
          'blue-end': '#00f2fe',
          'green-start': '#43e97b',
          'green-end': '#38f9d7',
          'purple-start': '#667eea',
          'purple-end': '#764ba2',
          'orange-start': '#fa709a',
          'orange-end': '#fee140',
          'pink-start': '#ff9a56',
          'pink-end': '#ff6b9d',
          'teal-start': '#a8edea',
          'teal-end': '#fed6e3',
        },
        // 玻璃拟态透明度
        glass: {
          white: 'rgba(255, 255, 255, 0.1)',
          border: 'rgba(255, 255, 255, 0.2)',
          dark: 'rgba(0, 0, 0, 0.1)',
        }
      },
      
      // 现代字体系统 - Inter字体
      fontFamily: {
        sans: ['Inter', 'ui-sans-serif', 'system-ui', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'Helvetica Neue', 'Arial', 'Noto Sans', 'sans-serif'],
        mono: ['JetBrains Mono', 'ui-monospace', 'SFMono-Regular', 'Menlo', 'Monaco', 'Consolas', 'Liberation Mono', 'Courier New', 'monospace'],
      },
      
      // iOS风格字体大小
      fontSize: {
        'display': ['3.75rem', { lineHeight: '1.2', letterSpacing: '-0.025em' }], // 60px
        'hero': ['3rem', { lineHeight: '1.2', letterSpacing: '-0.015em' }],       // 48px
        'title': ['2.25rem', { lineHeight: '1.3', letterSpacing: '-0.01em' }],    // 36px
        'subtitle': ['1.5rem', { lineHeight: '1.4' }],                           // 24px
        'body': ['1rem', { lineHeight: '1.6' }],                                 // 16px
        'caption': ['0.875rem', { lineHeight: '1.5' }],                          // 14px
        'small': ['0.75rem', { lineHeight: '1.4' }],                             // 12px
      },
      
      // 现代圆角系统
      borderRadius: {
        'xs': '0.125rem',    // 2px
        'sm': '0.25rem',     // 4px
        'md': '0.375rem',    // 6px
        'lg': '0.5rem',      // 8px
        'xl': '0.75rem',     // 12px
        '2xl': '1rem',       // 16px
        '3xl': '1.5rem',     // 24px
        '4xl': '2rem',       // 32px
        'card': '1rem',      // 卡片专用
        'button': '0.75rem', // 按钮专用
        'input': '0.5rem',   // 输入框专用
      },
      
      // iOS风格阴影系统
      boxShadow: {
        'ios-xs': '0 1px 2px 0 rgba(0, 0, 0, 0.05)',
        'ios-sm': '0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)',
        'ios-md': '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
        'ios-lg': '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
        'ios-xl': '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)',
        'glass': '0 8px 32px 0 rgba(31, 38, 135, 0.37)',
        'card': '0 4px 6px rgba(0, 0, 0, 0.05)',
        'card-hover': '0 20px 40px rgba(0, 0, 0, 0.15)',
        'button': '0 10px 25px rgba(102, 126, 234, 0.4)',
      },
      
      // 背景渐变配置
      backgroundImage: {
        'gradient-primary': 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
        'gradient-blue': 'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)',
        'gradient-green': 'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)',
        'gradient-orange': 'linear-gradient(135deg, #fa709a 0%, #fee140 100%)',
        'gradient-pink': 'linear-gradient(135deg, #ff9a56 0%, #ff6b9d 100%)',
        'gradient-purple': 'linear-gradient(135deg, #a8edea 0%, #fed6e3 100%)',
        'gradient-teal': 'linear-gradient(135deg, #a8edea 0%, #fed6e3 100%)',
        'hero-gradient': 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
        'glass-gradient': 'linear-gradient(145deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%)',
      },
      
      // 动画和过渡
      animation: {
        'float': 'float 3s ease-in-out infinite',
        'fade-in': 'fadeIn 0.5s ease-out',
        'slide-up': 'slideUp 0.5s ease-out',
        'scale-in': 'scaleIn 0.3s ease-out',
        'bounce-gentle': 'bounceGentle 2s infinite',
      },
      
      keyframes: {
        float: {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-10px)' },
        },
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { opacity: '0', transform: 'translateY(20px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        scaleIn: {
          '0%': { opacity: '0', transform: 'scale(0.9)' },
          '100%': { opacity: '1', transform: 'scale(1)' },
        },
        bounceGentle: {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-5px)' },
        },
      },
      
      // 间距系统优化
      spacing: {
        '18': '4.5rem',   // 72px
        '22': '5.5rem',   // 88px
        '26': '6.5rem',   // 104px
        '30': '7.5rem',   // 120px
        '34': '8.5rem',   // 136px
        '38': '9.5rem',   // 152px
        'safe-top': 'env(safe-area-inset-top)',
        'safe-bottom': 'env(safe-area-inset-bottom)',
        'safe-left': 'env(safe-area-inset-left)',
        'safe-right': 'env(safe-area-inset-right)',
      },
      
      // 背景模糊效果
      backdropBlur: {
        'xs': '2px',
        'glass': '10px',
        'strong': '16px',
      },
      
      // 缩放变换
      scale: {
        '102': '1.02',
        '105': '1.05',
      },
      
      // Z-index层级
      zIndex: {
        '60': '60',
        '70': '70',
        '80': '80',
        '90': '90',
        '100': '100',
      }
    },
  },
  plugins: [
    // 添加自定义插件支持玻璃拟态效果
    function({ addUtilities }) {
      const newUtilities = {
        '.glass-card': {
          background: 'rgba(255, 255, 255, 0.1)',
          backdropFilter: 'blur(10px)',
          border: '1px solid rgba(255, 255, 255, 0.2)',
        },
        '.glass-dark': {
          background: 'rgba(0, 0, 0, 0.1)',
          backdropFilter: 'blur(10px)',
          border: '1px solid rgba(0, 0, 0, 0.2)',
        },
        '.card-hover': {
          transition: 'all 0.3s ease',
        },
        '.card-hover:hover': {
          transform: 'translateY(-8px)',
          boxShadow: '0 20px 40px rgba(0, 0, 0, 0.15)',
        },
        '.btn-hover': {
          transition: 'all 0.3s ease',
        },
        '.btn-hover:hover': {
          transform: 'translateY(-1px)',
        },
        '.icon-gradient-blue': {
          background: 'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)',
        },
        '.icon-gradient-green': {
          background: 'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)',
        },
        '.icon-gradient-purple': {
          background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
        },
        '.icon-gradient-orange': {
          background: 'linear-gradient(135deg, #fa709a 0%, #fee140 100%)',
        },
        '.icon-gradient-pink': {
          background: 'linear-gradient(135deg, #ff9a56 0%, #ff6b9d 100%)',
        },
        '.icon-gradient-teal': {
          background: 'linear-gradient(135deg, #a8edea 0%, #fed6e3 100%)',
        },
        '.hero-text': {
          background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
          backgroundClip: 'text',
          WebkitBackgroundClip: 'text',
          WebkitTextFillColor: 'transparent',
        },
        '.stats-animation': {
          animation: 'float 3s ease-in-out infinite',
        },
      }
      addUtilities(newUtilities)
    }
  ],
} 