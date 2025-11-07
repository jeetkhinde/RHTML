// NotificationPartial - HTMX-friendly partial for dynamic notifications
@partial
struct NotificationProps {
    message: String,
    type: String,
}

NotificationPartial(NotificationProps { message, type }: NotificationProps) {
    <div class="notification notification-{type}" role="alert">
        <div class="notification-icon">
            <span r-if="type == 'success'">✓</span>
            <span r-else-if="type == 'error'">✗</span>
            <span r-else-if="type == 'warning'">!</span>
            <span r-else>ℹ</span>
        </div>
        <div class="notification-message">
            {message}
        </div>
    </div>
}

css NotificationPartial {
    .notification {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 1rem 1.25rem;
        border-radius: 0.5rem;
        margin: 0.5rem 0;
        animation: slideIn 0.3s ease-out;
    }

    @keyframes slideIn {
        from {
            transform: translateX(-100%);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }

    .notification-success {
        background-color: #d1fae5;
        border: 1px solid #10b981;
        color: #065f46;
    }

    .notification-error {
        background-color: #fee2e2;
        border: 1px solid #ef4444;
        color: #991b1b;
    }

    .notification-warning {
        background-color: #fef3c7;
        border: 1px solid #f59e0b;
        color: #78350f;
    }

    .notification-info {
        background-color: #dbeafe;
        border: 1px solid #3b82f6;
        color: #1e40af;
    }

    .notification-icon {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 24px;
        height: 24px;
        font-size: 1.25rem;
        font-weight: bold;
    }

    .notification-message {
        flex: 1;
        font-size: 0.875rem;
        line-height: 1.5;
    }
}
