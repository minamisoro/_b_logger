use blogger_api::db::establish_connection_pool;
use blogger_api::models::*;
use blogger_api::schema::*;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use dotenvy::dotenv;
use fake::Fake;
use fake::faker::internet::en::*;
use fake::faker::lorem::en::*;
use fake::faker::name::en::*;
use rand::Rng;
use rand::seq::SliceRandom;
use std::env;
use uuid::Uuid;

const NUM_USERS: usize = 20;
const NUM_POSTS: usize = 100;
const NUM_USER_GROUPS: usize = 5;
const NUM_COMMENTS: usize = 200;

fn main() {
    println!("🌱 Starting database seeding...\n");

    // Load environment variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create connection pool and get connection
    let pool = establish_connection_pool(&database_url);
    let mut conn = pool.get().expect("Failed to get database connection");

    // Reset database - delete all existing data
    println!("🗑️  Resetting database...");
    reset_database(&mut conn);
    println!("✅ Database reset complete\n");

    // Seed in order due to foreign key constraints
    println!("👥 Seeding users...");
    let users = seed_users(&mut conn);
    println!("✅ Created {} users\n", users.len());

    println!("🎭 Seeding roles...");
    let roles = seed_roles(&mut conn);
    println!("✅ Created {} roles\n", roles.len());

    println!("🔗 Assigning user roles...");
    seed_user_roles(&mut conn, &users, &roles);
    println!("✅ Assigned user roles\n");

    println!("🏷️  Seeding tags...");
    let tags = seed_tags(&mut conn);
    println!("✅ Created {} tags\n", tags.len());

    println!("📝 Seeding posts...");
    let posts = seed_posts(&mut conn, &users);
    println!("✅ Created {} posts\n", posts.len());

    println!("🔖 Assigning tags to posts...");
    seed_post_tags(&mut conn, &posts, &tags);
    println!("✅ Assigned tags to posts\n");

    println!("🔗 Creating post links...");
    seed_post_links(&mut conn, &posts);
    println!("✅ Created post links\n");

    println!("👥 Seeding user groups...");
    let groups = seed_user_groups(&mut conn, &users);
    println!("✅ Created {} user groups\n", groups.len());

    println!("💬 Seeding comments...");
    seed_comments(&mut conn, &posts, &users);
    println!("✅ Created {} comments\n", NUM_COMMENTS);

    println!("\n🎉 Database seeding completed successfully!");
}

fn reset_database(conn: &mut PgConnection) {
    // Delete all data in reverse order of foreign key dependencies
    // This ensures we don't violate foreign key constraints during deletion

    println!("  Deleting comments...");
    diesel::delete(comments::table)
        .execute(conn)
        .expect("Error deleting comments");

    println!("  Deleting post links...");
    diesel::delete(post_links::table)
        .execute(conn)
        .expect("Error deleting post links");

    println!("  Deleting post tags...");
    diesel::delete(post_tags::table)
        .execute(conn)
        .expect("Error deleting post tags");

    println!("  Deleting posts...");
    diesel::delete(posts::table)
        .execute(conn)
        .expect("Error deleting posts");

    println!("  Deleting tags...");
    diesel::delete(tags::table)
        .execute(conn)
        .expect("Error deleting tags");

    println!("  Deleting user group members...");
    diesel::delete(user_group_members::table)
        .execute(conn)
        .expect("Error deleting user group members");

    println!("  Deleting user groups...");
    diesel::delete(user_groups::table)
        .execute(conn)
        .expect("Error deleting user groups");

    println!("  Deleting OpenID credentials...");
    diesel::delete(open_id_credentials::table)
        .execute(conn)
        .expect("Error deleting OpenID credentials");

    println!("  Deleting user roles...");
    diesel::delete(user_roles::table)
        .execute(conn)
        .expect("Error deleting user roles");

    println!("  Deleting roles...");
    diesel::delete(roles::table)
        .execute(conn)
        .expect("Error deleting roles");

    println!("  Deleting users...");
    diesel::delete(users::table)
        .execute(conn)
        .expect("Error deleting users");
}

fn seed_users(conn: &mut PgConnection) -> Vec<User> {
    let mut users = Vec::new();

    for _ in 0..NUM_USERS {
        let username: String = Username().fake();
        let email: String = SafeEmail().fake();
        let display_name: String = Name().fake();

        let new_user = NewUser {
            username: username.to_lowercase(),
            email: email.to_lowercase(),
            display_name: Some(display_name),
        };

        let user: User = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .expect("Error inserting user");

        users.push(user);
    }

    users
}

fn seed_roles(conn: &mut PgConnection) -> Vec<Role> {
    let role_names = vec![
        ("admin", "System administrator with full access"),
        ("editor", "Can create and edit all content"),
        ("author", "Can create and edit own posts"),
        ("moderator", "Can moderate comments and users"),
        ("viewer", "Read-only access"),
    ];

    let mut roles = Vec::new();

    for (name, description) in role_names {
        let new_role = NewRole {
            name: name.to_string(),
            description: Some(description.to_string()),
        };

        let role: Role = diesel::insert_into(roles::table)
            .values(&new_role)
            .get_result(conn)
            .expect("Error inserting role");

        roles.push(role);
    }

    roles
}

fn seed_user_roles(conn: &mut PgConnection, users: &[User], roles: &[Role]) {
    let mut rng = rand::thread_rng();

    for user in users {
        // Assign 1-3 random roles to each user
        let num_roles = rng.gen_range(1..=3);
        let selected_roles: Vec<_> = roles.choose_multiple(&mut rng, num_roles).collect();

        for role in selected_roles {
            let new_user_role = NewUserRole {
                user_id: user.id,
                role_id: role.id,
            };

            diesel::insert_into(user_roles::table)
                .values(&new_user_role)
                .on_conflict_do_nothing()
                .execute(conn)
                .expect("Error inserting user role");
        }
    }
}

fn seed_tags(conn: &mut PgConnection) -> Vec<Tag> {
    let tag_names = vec![
        "rust",
        "python",
        "javascript",
        "typescript",
        "go",
        "java",
        "web-development",
        "backend",
        "frontend",
        "database",
        "devops",
        "machine-learning",
        "data-science",
        "cloud",
        "aws",
        "docker",
        "kubernetes",
        "microservices",
        "api",
        "rest",
        "graphql",
        "security",
        "performance",
        "testing",
        "tutorial",
        "guide",
        "best-practices",
        "architecture",
        "design-patterns",
        "algorithms",
    ];

    let mut tags = Vec::new();
    let mut parent_tags = Vec::new();

    // Create root tags first
    for &name in tag_names.iter().take(10) {
        let new_tag = NewTag {
            name: name.to_string(),
            slug: name.to_string(),
            description: Some(format!("Posts related to {}", name)),
            parent_id: None,
        };

        let tag: Tag = diesel::insert_into(tags::table)
            .values(&new_tag)
            .get_result(conn)
            .expect("Error inserting tag");

        parent_tags.push(tag.clone());
        tags.push(tag);
    }

    // Create child tags
    let mut rng = rand::thread_rng();
    for &name in tag_names.iter().skip(10) {
        let parent = parent_tags.choose(&mut rng).map(|t| t.id);

        let new_tag = NewTag {
            name: name.to_string(),
            slug: name.to_string(),
            description: Some(format!("Posts related to {}", name)),
            parent_id: parent,
        };

        let tag: Tag = diesel::insert_into(tags::table)
            .values(&new_tag)
            .get_result(conn)
            .expect("Error inserting tag");

        tags.push(tag);
    }

    tags
}

fn seed_posts(conn: &mut PgConnection, users: &[User]) -> Vec<Post> {
    let mut rng = rand::thread_rng();
    let mut posts = Vec::new();

    for i in 0..NUM_POSTS {
        let author = users.choose(&mut rng).unwrap();
        let title: String = Sentence(3..8).fake();
        let slug = title
            .to_lowercase()
            .replace(" ", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();

        // Generate realistic content
        let num_paragraphs = rng.gen_range(3..10);
        let paragraphs: Vec<String> = (0..num_paragraphs)
            .map(|_| Paragraph(5..15).fake::<String>())
            .collect();
        let content = paragraphs.join("\n\n");

        // 80% published, 20% draft
        let published = rng.gen_bool(0.8);
        let visibility = if rng.gen_bool(0.9) {
            "public"
        } else {
            "private"
        };

        // Random publish date in the past year
        let days_ago = rng.gen_range(0..365);
        let published_at = if published {
            Some(Utc::now().naive_utc() - Duration::days(days_ago))
        } else {
            None
        };

        let new_post = NewPost {
            author_id: author.id,
            title: title.clone(),
            slug: format!("{}-{}", slug, i),
            content,
            published,
            visibility: visibility.to_string(),
            published_at,
        };

        let post: Post = diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn)
            .expect("Error inserting post");

        posts.push(post);
    }

    posts
}

fn seed_post_tags(conn: &mut PgConnection, posts: &[Post], tags: &[Tag]) {
    let mut rng = rand::thread_rng();

    for post in posts {
        // Assign 1-5 random tags to each post
        let num_tags = rng.gen_range(1..=5);
        let selected_tags: Vec<_> = tags.choose_multiple(&mut rng, num_tags).collect();

        for tag in selected_tags {
            let new_post_tag = NewPostTag {
                post_id: post.id,
                tag_id: tag.id,
            };

            diesel::insert_into(post_tags::table)
                .values(&new_post_tag)
                .on_conflict_do_nothing()
                .execute(conn)
                .expect("Error inserting post tag");
        }
    }
}

fn seed_post_links(conn: &mut PgConnection, posts: &[Post]) {
    let mut rng = rand::thread_rng();
    let num_links = posts.len() / 5; // Create links for ~20% of posts

    for _ in 0..num_links {
        let source = posts.choose(&mut rng).unwrap();
        let target = posts.choose(&mut rng).unwrap();

        // Don't link to self
        if source.id == target.id {
            continue;
        }

        let new_link = NewPostLink {
            source_post_id: source.id,
            target_post_id: target.id,
        };

        diesel::insert_into(post_links::table)
            .values(&new_link)
            .on_conflict_do_nothing()
            .execute(conn)
            .expect("Error inserting post link");
    }
}

fn seed_user_groups(conn: &mut PgConnection, users: &[User]) -> Vec<UserGroup> {
    let mut rng = rand::thread_rng();
    let mut groups = Vec::new();

    let group_names = vec![
        "Rust Enthusiasts",
        "Web Development Team",
        "Backend Engineers",
        "Frontend Developers",
        "DevOps Circle",
        "Data Science Group",
        "Mobile Developers",
    ];

    for name in group_names.iter().take(NUM_USER_GROUPS) {
        let owner = users.choose(&mut rng).unwrap();

        let new_group = NewUserGroup {
            user_id: owner.id,
            name: name.to_string(),
        };

        let group: UserGroup = diesel::insert_into(user_groups::table)
            .values(&new_group)
            .get_result(conn)
            .expect("Error inserting user group");

        // Add 3-8 random members to each group
        let num_members = rng.gen_range(3..=8);
        let members: Vec<_> = users.choose_multiple(&mut rng, num_members).collect();

        for member in members {
            let new_member = NewUserGroupMember {
                group_id: group.id,
                user_id: member.id,
            };

            diesel::insert_into(user_group_members::table)
                .values(&new_member)
                .on_conflict_do_nothing()
                .execute(conn)
                .expect("Error inserting group member");
        }

        groups.push(group);
    }

    groups
}

fn seed_comments(conn: &mut PgConnection, posts: &[Post], users: &[User]) {
    let mut rng = rand::thread_rng();

    // Only comment on published posts
    let published_posts: Vec<_> = posts.iter().filter(|p| p.published).collect();

    for _ in 0..NUM_COMMENTS {
        let post = published_posts.choose(&mut rng).unwrap();
        let author = users.choose(&mut rng).unwrap();

        let content: String = Paragraph(2..6).fake();
        let is_approved = rng.gen_bool(0.9); // 90% approved

        // 20% chance of being a reply to another comment on the same post
        let parent_comment_id = if rng.gen_bool(0.2) {
            comments::table
                .filter(comments::post_id.eq(post.id))
                .select(comments::id)
                .first::<Uuid>(conn)
                .ok()
        } else {
            None
        };

        let new_comment = NewComment {
            post_id: post.id,
            author_id: author.id,
            parent_comment_id,
            content,
            is_approved,
        };

        diesel::insert_into(comments::table)
            .values(&new_comment)
            .execute(conn)
            .expect("Error inserting comment");
    }
}

// Helper structs for insertions
#[derive(Insertable)]
#[diesel(table_name = roles)]
struct NewRole {
    name: String,
    description: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = user_roles)]
struct NewUserRole {
    user_id: Uuid,
    role_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
struct NewTag {
    name: String,
    slug: String,
    description: Option<String>,
    parent_id: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
struct NewPost {
    author_id: Uuid,
    title: String,
    slug: String,
    content: String,
    published: bool,
    visibility: String,
    published_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = post_tags)]
struct NewPostTag {
    post_id: Uuid,
    tag_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = post_links)]
struct NewPostLink {
    source_post_id: Uuid,
    target_post_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
struct NewComment {
    post_id: Uuid,
    author_id: Uuid,
    parent_comment_id: Option<Uuid>,
    content: String,
    is_approved: bool,
}
