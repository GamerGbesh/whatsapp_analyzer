// Mirror of the Rust WhatsResult / UserStat shapes.

export type Chat = { message: string; date: string };

export type User = {
  name: string;
  chats?: Chat[];
};

export type UserStat = {
  user: User;
  messages_per_day: Record<string, number>;
  messages_per_month: Record<string, number>;
  busiest_day: [string, number] | null;
  busiest_month: [number, number] | null;
  first_message: string | null;
  last_message: string | null;
  total_chats: number;
  avg_message_length: number;
};

export type WhatsResult = {
  most_active_user: string | null;
  user_results: UserStat[];
};
