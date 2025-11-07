cmp Card {
  <div class="card">
    <h3 class="card-title">{title}</h3>
    <p class="card-description">{description}</p>
  </div>
}

css Card {
  .card {
    background: white;
    border-radius: 12px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 12px rgba(0, 0, 0, 0.15);
  }

  .card-title {
    font-size: 1.25rem;
    font-weight: 700;
    color: #1a202c;
    margin-bottom: 0.5rem;
  }

  .card-description {
    color: #718096;
    line-height: 1.6;
  }
}
