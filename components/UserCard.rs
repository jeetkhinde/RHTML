// UserCard Component - Function-based syntax with conditionals
struct UserCardProps {
    name: String,
    email: String,
    avatar: String,
    status: String,
    role: String,
    show_actions: bool,
}

UserCard(UserCardProps { name, email, avatar, status, role, show_actions }: UserCardProps) {
    <div class="user-card">
        <div class="user-avatar">
            <img src="{avatar}" alt="{name}" />
        </div>
        <div class="user-info">
            <h3 class="user-name">{name}</h3>
            <p class="user-email">{email}</p>
            <p class="user-role">{role}</p>
            <div r-component="StatusBadge" label="{status}" color="green" />
        </div>
        <div class="actions" r-if="show_actions">
            <button class="btn-edit">Edit</button>
            <button class="btn-delete">Delete</button>
        </div>
    </div>
}

css UserCard {
    .user-card {
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 1.5rem;
        background: white;
        border-radius: 12px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        transition: transform 0.2s, box-shadow 0.2s;
    }

    .user-card:hover {
        transform: translateY(-2px);
        box-shadow: 0 6px 12px rgba(0, 0, 0, 0.15);
    }

    .user-avatar {
        flex-shrink: 0;
    }

    .user-avatar img {
        width: 64px;
        height: 64px;
        border-radius: 50%;
        object-fit: cover;
    }

    .user-info {
        flex: 1;
    }

    .user-name {
        font-size: 1.25rem;
        font-weight: 600;
        margin: 0 0 0.25rem 0;
        color: #111827;
    }

    .user-email {
        font-size: 0.875rem;
        color: #6b7280;
        margin: 0 0 0.25rem 0;
    }

    .user-role {
        font-size: 0.75rem;
        color: #9ca3af;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        margin: 0 0 0.5rem 0;
    }

    .actions {
        display: flex;
        gap: 0.5rem;
    }

    .btn-edit, .btn-delete {
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 0.375rem;
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
        transition: opacity 0.2s;
    }

    .btn-edit {
        background-color: #3b82f6;
        color: white;
    }

    .btn-edit:hover {
        opacity: 0.9;
    }

    .btn-delete {
        background-color: #ef4444;
        color: white;
    }

    .btn-delete:hover {
        opacity: 0.9;
    }
}
