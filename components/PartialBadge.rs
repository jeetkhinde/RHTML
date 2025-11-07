// PartialBadge - Accessible as a partial (public endpoint)
@partial
struct PartialBadgeProps {
    label: String,
    color: String,
}

PartialBadge(PartialBadgeProps { label, color }: PartialBadgeProps) {
    <span class="partial-badge badge-{color}">
        {label}
    </span>
}

css PartialBadge {
    .partial-badge {
        display: inline-block;
        padding: 0.5rem 1rem;
        font-size: 0.875rem;
        font-weight: 600;
        border-radius: 0.5rem;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .badge-success {
        background: linear-gradient(135deg, #10b981 0%, #059669 100%);
        color: white;
    }

    .badge-warning {
        background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
        color: #111827;
    }

    .badge-danger {
        background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
        color: white;
    }

    .badge-info {
        background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
        color: white;
    }
}
