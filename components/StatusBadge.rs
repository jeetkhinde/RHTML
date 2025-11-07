// StatusBadge Component - Function-based syntax
struct StatusBadgeProps {
    label: String,
    color: String,
}

StatusBadge(StatusBadgeProps { label, color }: StatusBadgeProps) {
    <span class="status-badge status-{color}">
        {label}
    </span>
}

css StatusBadge {
    .status-badge {
        display: inline-block;
        padding: 0.25rem 0.75rem;
        font-size: 0.875rem;
        font-weight: 600;
        border-radius: 0.375rem;
        text-transform: uppercase;
        letter-spacing: 0.025em;
    }

    .status-green {
        background-color: #10b981;
        color: white;
    }

    .status-blue {
        background-color: #3b82f6;
        color: white;
    }

    .status-red {
        background-color: #ef4444;
        color: white;
    }

    .status-yellow {
        background-color: #f59e0b;
        color: #111827;
    }
}
