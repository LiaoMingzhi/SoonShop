import React from 'react';
import { ModalProps } from '../../types';

const Modal: React.FC<ModalProps> = ({
  visible,
  title,
  width = 600,
  onClose,
  onConfirm,
  confirmText = '确认',
  cancelText = '取消',
  children
}) => {
  if (!visible) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* 背景遮罩 */}
      <div 
        className="fixed inset-0 bg-black bg-opacity-50 transition-opacity"
        onClick={onClose}
      />
      
      {/* 模态框内容 */}
      <div 
        className="relative bg-white rounded-2xl shadow-xl max-h-[90vh] overflow-y-auto"
        style={{ width: `${width}px`, maxWidth: '90vw' }}
      >
        {/* 头部 */}
        {title && (
          <div className="flex items-center justify-between p-6 border-b border-gray-200">
            <h2 className="text-xl font-semibold text-gray-800">{title}</h2>
            <button
              onClick={onClose}
              className="text-gray-400 hover:text-gray-600 transition-colors"
            >
              <i className="fas fa-times text-xl"></i>
            </button>
          </div>
        )}
        
        {/* 内容 */}
        <div className="p-6">
          {children}
        </div>
        
        {/* 底部按钮 */}
        {onConfirm && (
          <div className="flex justify-end space-x-3 p-6 border-t border-gray-200">
            <button
              onClick={onClose}
              className="px-4 py-2 text-gray-600 hover:text-gray-800 transition-colors"
            >
              {cancelText}
            </button>
            <button
              onClick={onConfirm}
              className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
            >
              {confirmText}
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default Modal; 