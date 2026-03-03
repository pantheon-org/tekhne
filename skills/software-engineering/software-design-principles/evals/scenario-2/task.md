# Refactoring: UserManagement Service

## Problem/Feature Description

Your team inherited a legacy `UserManagement` service that has become a maintenance nightmare. The single class handles user registration, authentication, profile management, password reset, session management, and analytics. Every time any of these features needs to change, multiple developers end up in conflicts during code reviews.

The CTO has approved a refactoring effort to improve maintainability. Your task is to analyze the current implementation and provide a refactoring plan that will make the code easier to maintain, test, and extend.

**Current issues reported by the team:**

- Adding a new OAuth provider takes 2 weeks and breaks existing functionality
- Unit tests are impossible to write - everything depends on concrete implementations
- The password reset flow shares code with registration in ways that cause bugs
- Session management changes accidentally affect user profile updates

## Output Specification

Create a refactoring plan document called `refactoring-plan.md` that:

1. **Classifies** this refactoring effort (architectural, tactical, or foundational)
2. **Applies SOLID principles** to identify what needs to change:
   - Which responsibilities should be split (SRP)
   - How to make the code extensible for new auth providers (OCP)
   - Where to introduce abstractions (DIP)
3. **Evaluates pattern needs**: Should you use Factory for provider creation? Observer for events? Strategy for authentication? Justify each pattern choice with explicit benefit/cost analysis
4. **Avoids pitfalls**:
   - Does NOT keep the god class structure
   - Does NOT add caching or optimization without evidence
   - Does NOT design for imagined future OAuth providers (YAGNI)
5. **Documents the plan** using BAD/GOOD format for current vs. proposed code
6. **Documents tradeoffs** of the refactoring approach and risks

## Input Files (optional)

=============== FILE: user-management.ts ===============
export class UserManagementService {
  private db: any;
  private emailSender: any;
  private sessionStore: any;
  private analytics: any;
  private passwordHasher: any;

  constructor() {
    this.db = require('./database');
    this.emailSender = require('./email-service');
    this.sessionStore = require('./session-store');
    this.analytics = require('./analytics');
    this.passwordHasher = require('./bcrypt');
  }

  async registerUser(email: string, password: string, profile: any) {
    // Validation
    if (!email || !password) {
      throw new Error('Email and password required');
    }

    // Check if user exists
    const existing = await this.db.users.findByEmail(email);
    if (existing) {
      throw new Error('User already exists');
    }

    // Hash password
    const hashedPassword = await this.passwordHasher.hash(password);

    // Create user
    const user = await this.db.users.create({
      email,
      password: hashedPassword,
      profile,
      createdAt: new Date(),
    });

    // Send welcome email
    await this.emailSender.send(email, 'welcome', { email });

    // Track in analytics
    await this.analytics.track(user.id, 'user_registered', { email });

    return user;
  }

  async login(email: string, password: string) {
    const user = await this.db.users.findByEmail(email);
    if (!user) {
      throw new Error('Invalid credentials');
    }

    const valid = await this.passwordHasher.compare(password, user.password);
    if (!valid) {
      throw new Error('Invalid credentials');
    }

    // Create session
    const sessionId = await this.sessionStore.create(user.id, {
      email: user.email,
      role: user.role,
    });

    await this.analytics.track(user.id, 'user_logged_in', {});

    return { user, sessionId };
  }

  async resetPassword(email: string) {
    const user = await this.db.users.findByEmail(email);
    if (!user) {
      return; // Silent fail for security
    }

    const resetToken = crypto.randomBytes(32).toString('hex');
    await this.db.passwordResets.create({
      userId: user.id,
      token: resetToken,
      expiresAt: new Date(Date.now() + 3600000),
    });

    await this.emailSender.send(email, 'password-reset', {
      resetLink: `https://app.example.com/reset?token=${resetToken}`,
    });
  }

  async updateProfile(userId: string, updates: any) {
    const user = await this.db.users.findById(userId);
    if (!user) {
      throw new Error('User not found');
    }

    // Update in database
    const updated = await this.db.users.update(userId, updates);

    // Invalidate sessions if email changed
    if (updates.email) {
      await this.sessionStore.invalidateByUserId(userId);
    }

    // Track change
    await this.analytics.track(userId, 'profile_updated', updates);

    return updated;
  }

  async authenticateWithOAuth(provider: string, token: string) {
    // This method has grown organically
    let userInfo: any;

    if (provider === 'google') {
      const googleUser = await this.getGoogleUser(token);
      userInfo = { email: googleUser.email, name: googleUser.name };
    } else if (provider === 'facebook') {
      const fbUser = await this.getFacebookUser(token);
      userInfo = { email: fbUser.email, name: fbUser.name };
    } else if (provider === 'github') {
      const ghUser = await this.getGithubUser(token);
      userInfo = { email: ghUser.email, name: ghUser.name };
    } else {
      // What happens when we add LinkedIn, Twitter, etc?
      // This if-else chain keeps growing
      throw new Error('Unsupported provider');
    }

    // Find or create user
    let user = await this.db.users.findByEmail(userInfo.email);
    if (!user) {
      user = await this.db.users.create({
        email: userInfo.email,
        profile: { name: userInfo.name },
        oauthProvider: provider,
      });
    }

    const sessionId = await this.sessionStore.create(user.id, {
      email: user.email,
      role: user.role,
      oauthProvider: provider,
    });

    return { user, sessionId };
  }

  private async getGoogleUser(token: string) {
    // Implementation
    return { email: `user@gmail.com`, name: 'Google User' };
  }

  private async getFacebookUser(token: string) {
    // Implementation
    return { email: `user@facebook.com`, name: 'FB User' };
  }

  private async getGithubUser(token: string) {
    // Implementation
    return { email: `user@github.com`, name: 'GitHub User' };
  }

  async logout(userId: string, sessionId: string) {
    await this.sessionStore.destroy(sessionId);
    await this.analytics.track(userId, 'user_logged_out', {});
  }

  async getUserAnalytics(userId: string) {
    const sessions = await this.sessionStore.getByUserId(userId);
    const logins = await this.analytics.getEvents(userId, 'user_logged_in');
    const updates = await this.analytics.getEvents(userId, 'profile_updated');

    return {
      sessionCount: sessions.length,
      loginCount: logins.length,
      profileUpdateCount: updates.length,
      lastLogin: logins[0]?.timestamp,
    };
  }
}
