use std::cmp::{ PartialEq, PartialOrd, Eq, Ord, Ordering };
use std::fmt::{ self, Display, Debug, Formatter };
use std::{ slice, vec };

struct Deserialiser {
	// TODO: ?
}

impl Deserialiser {
	/*
	const uint8_t *read_ptr;
	const uint8_t *end_ptr;

	public:
	inline Deserializer(const std::vector<uint8_t> &input) :
		read_ptr(input.data()),
		end_ptr(input.data() + input.size()) {};

	template <typename T>
	T peek() const {
		T value = 0;
		const uint8_t *temp_ptr = read_ptr;
		if (static_cast<unsigned>(end_ptr - temp_ptr) >= sizeof(T)) {
			for (auto i = 0u; i < sizeof(T); i++) {
				value |= static_cast<T>(*(temp_ptr++)) << static_cast<T>(8 * i);
			}
		}
		return value;
	}

	template <typename T>
	T read() {
		T value = peek<T>();
		read_ptr += sizeof(T);
		return value;
	}
	*/
}

struct EncodingConversion {
	// TODO: ?
}

impl EncodingConversion {
	/*
	void *data;
	int mode;

	EncodingConversion(int, void *);
	int convert(const char **, const char *, char **, char *) const;

	public:
	EncodingConversion(EncodingConversion &&);
	EncodingConversion();
	~EncodingConversion();

	bool encode(const std::u16string &, size_t start_offset, size_t end_offset, FILE *stream, std::vector<char> &buffer);
	size_t encode(const std::u16string &, size_t *start_offset, size_t end_offset, char *buffer, size_t buffer_size, bool is_last = false);
	bool decode(std::u16string &, FILE *stream, std::vector<char> &buffer, std::function<void(size_t)> progress_callback);
	size_t decode(std::u16string &, const char *buffer, size_t buffer_size, bool is_last = false);

	friend optional<EncodingConversion> transcoding_to(const char *);
	friend optional<EncodingConversion> transcoding_from(const char *);
	*/
}

// TODO: ???????? whatisthis and why does it not just use hash set or something
#[derive(Clone)]
struct FlatSet<T> {
	contents: Vec<T>
}

impl<T: Ord> FlatSet<T> {
	fn new() -> Self {
		let contents = Vec::new();
		Self { contents }
	}
	fn with_capacity(capacity: usize) -> Self {
		let contents = Vec::with_capacity(capacity);
		Self { contents }
	}

	fn insert(&mut self, value: T) {
		// TODO: ?????????????????????????? WHY NOT B TREE SET OR SOMETHING IT MAINTAINS ORDER LOL
		if let Err(i) = self.contents.binary_search(&value) {
			self.contents.insert(i, value);
		}
	}

	fn erase(&mut self, value: &T) -> Option<T> {
		self.contents
			.binary_search(value)
			.map(|i| self.contents.remove(i))
			.ok()
	}

	fn iter(&self) -> slice::Iter<T> {
		self.contents.iter()
	}

	fn iter_mut(&mut self) -> slice::IterMut<T> {
		self.contents.iter_mut()
	}

	fn into_iter(self) -> vec::IntoIter<T> {
		self.contents.into_iter()
	}

	fn count(&self, value: &T) -> usize {
		self.contents.binary_search(value).is_ok() as _
	}

	fn len(&self) -> usize {
		self.contents.len()
	}

	fn is_empty(&self) -> bool {
		self.contents.is_empty()
	}
}

impl<T: Ord> Extend<T> for FlatSet<T> {
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		iter.into_iter().for_each(|value| self.insert(value));
	}
}

// TODO: that libmba diff thing?

struct MarkerIndex {
	// TODO: ?
}

impl MarkerIndex {
	/*
	public:
	using MarkerId = unsigned;
	using MarkerIdSet = flat_set<MarkerId>;

	struct SpliceResult {
		flat_set<MarkerId> touch;
		flat_set<MarkerId> inside;
		flat_set<MarkerId> overlap;
		flat_set<MarkerId> surround;
	};

	struct Boundary {
		Point position;
		flat_set<MarkerId> starting;
		flat_set<MarkerId> ending;
	};

	struct BoundaryQueryResult {
		std::vector<MarkerId> containing_start;
		std::vector<Boundary> boundaries;
	};

	MarkerIndex(unsigned seed = 0u);
	~MarkerIndex();
	int generate_random_number();
	void insert(MarkerId id, Point start, Point end);
	void set_exclusive(MarkerId id, bool exclusive);
	void remove(MarkerId id);
	bool has(MarkerId id);
	SpliceResult splice(Point start, Point old_extent, Point new_extent);
	Point get_start(MarkerId id) const;
	Point get_end(MarkerId id) const;
	Range get_range(MarkerId id) const;

	int compare(MarkerId id1, MarkerId id2) const;
	flat_set<MarkerId> find_intersecting(Point start, Point end);
	flat_set<MarkerId> find_containing(Point start, Point end);
	flat_set<MarkerId> find_contained_in(Point start, Point end);
	flat_set<MarkerId> find_starting_in(Point start, Point end);
	flat_set<MarkerId> find_starting_at(Point position);
	flat_set<MarkerId> find_ending_in(Point start, Point end);
	flat_set<MarkerId> find_ending_at(Point position);
	BoundaryQueryResult find_boundaries_after(Point start, size_t max_count);

	std::unordered_map<MarkerId, Range> dump();

	private:
	friend class Iterator;

	struct Node {
		Node *parent;
		Node *left;
		Node *right;
		Point left_extent;
		flat_set<MarkerId> left_marker_ids;
		flat_set<MarkerId> right_marker_ids;
		flat_set<MarkerId> start_marker_ids;
		flat_set<MarkerId> end_marker_ids;
		int priority;

		Node(Node *parent, Point left_extent);
		bool is_marker_endpoint();
	};

	class Iterator {
	public:
		Iterator(MarkerIndex *marker_index);
		void reset();
		Node* insert_marker_start(const MarkerId &id, const Point &start_position, const Point &end_position);
		Node* insert_marker_end(const MarkerId &id, const Point &start_position, const Point &end_position);
		Node* insert_splice_boundary(const Point &position, bool is_insertion_end);
		void find_intersecting(const Point &start, const Point &end, flat_set<MarkerId> *result);
		void find_contained_in(const Point &start, const Point &end, flat_set<MarkerId> *result);
		void find_starting_in(const Point &start, const Point &end, flat_set<MarkerId> *result);
		void find_ending_in(const Point &start, const Point &end, flat_set<MarkerId> *result);
		void find_boundaries_after(Point start, size_t max_count, BoundaryQueryResult *result);
		std::unordered_map<MarkerId, Range> dump();

	private:
		void ascend();
		void descend_left();
		void descend_right();
		void move_to_successor();
		void seek_to_first_node_greater_than_or_equal_to(const Point &position);
		void mark_right(const MarkerId &id, const Point &start_position, const Point &end_position);
		void mark_left(const MarkerId &id, const Point &start_position, const Point &end_position);
		Node* insert_left_child(const Point &position);
		Node* insert_right_child(const Point &position);
		void check_intersection(const Point &start, const Point &end, flat_set<MarkerId> *results);
		void cache_node_position() const;

		MarkerIndex *marker_index;
		Node *current_node;
		Point current_node_position;
		Point left_ancestor_position;
		Point right_ancestor_position;
		std::vector<Point> left_ancestor_position_stack;
		std::vector<Point> right_ancestor_position_stack;
	};

	Point get_node_position(const Node *node) const;
	void delete_node(Node *node);
	void delete_subtree(Node *node);
	void bubble_node_up(Node *node);
	void bubble_node_down(Node *node);
	void rotate_node_left(Node *pivot);
	void rotate_node_right(Node *pivot);
	void get_starting_and_ending_markers_within_subtree(const Node *node, flat_set<MarkerId> *starting, flat_set<MarkerId> *ending);
	void populate_splice_invalidation_sets(SpliceResult *invalidated, const Node *start_node, const Node *end_node, const flat_set<MarkerId> &starting_inside_splice, const flat_set<MarkerId> &ending_inside_splice);

	std::default_random_engine random_engine;
	std::uniform_int_distribution<int> random_distribution;
	Node *root;
	std::unordered_map<MarkerId, Node*> start_nodes_by_id;
	std::unordered_map<MarkerId, Node*> end_nodes_by_id;
	Iterator iterator;
	flat_set<MarkerId> exclusive_marker_ids;
	mutable std::unordered_map<const Node*, Point> node_position_cache;
	*/
}

struct Patch {
	// TODO: ?
}

impl Patch {
	/*
	struct Node;
	struct OldCoordinates;
	struct NewCoordinates;
	struct PositionStackEntry;

	Node *root;
	std::vector<Node *> node_stack;
	std::vector<PositionStackEntry> left_ancestor_stack;
	uint32_t change_count;
	bool merges_adjacent_changes;

	public:
	struct Change {
		Point old_start;
		Point old_end;
		Point new_start;
		Point new_end;
		Text *old_text;
		Text *new_text;
		uint32_t preceding_old_text_size;
		uint32_t preceding_new_text_size;
		uint32_t old_text_size;
	};

	// Construction and destruction
	Patch(bool merges_adjacent_changes = true);
	Patch(Patch &&);
	Patch(Deserializer &input);
	Patch &operator=(Patch &&);
	~Patch();
	void serialize(Serializer &serializer);

	Patch copy();
	Patch invert();

	// Mutations
	bool splice(Point new_splice_start,
							Point new_deletion_extent, Point new_insertion_extent,
							optional<Text> &&deleted_text = optional<Text>{},
							optional<Text> &&inserted_text = optional<Text>{},
							uint32_t deleted_text_size = 0);
	void splice_old(Point start, Point deletion_extent, Point insertion_extent);
	bool combine(const Patch &other, bool left_to_right = true);
	void clear();
	void rebalance();

	// Non-splaying reads
	std::vector<Change> get_changes() const;
	size_t get_change_count() const;
	std::vector<Change> get_changes_in_old_range(Point start, Point end) const;
	std::vector<Change> get_changes_in_new_range(Point start, Point end) const;
	optional<Change> get_change_starting_before_old_position(Point position) const;
	optional<Change> get_change_starting_before_new_position(Point position) const;
	optional<Change> get_change_ending_after_new_position(Point position) const;
	optional<Change> get_bounds() const;
	Point new_position_for_new_offset(uint32_t new_offset,
																		std::function<uint32_t(Point)> old_offset_for_old_position,
																		std::function<Point(uint32_t)> old_position_for_old_offset) const;

	// Splaying reads
	std::vector<Change> grab_changes_in_old_range(Point start, Point end);
	std::vector<Change> grab_changes_in_new_range(Point start, Point end);
	optional<Change> grab_change_starting_before_old_position(Point position);
	optional<Change> grab_change_starting_before_new_position(Point position);
	optional<Change> grab_change_ending_after_new_position(Point position, bool exclusive = false);

	// Debugging
	std::string get_dot_graph() const;
	std::string get_json() const;

	private:
	Patch(Node *root, uint32_t change_count, bool merges_adjacent_changes);

	template <typename CoordinateSpace>
	std::vector<Change> get_changes_in_range(Point, Point, bool inclusive) const;

	template <typename CoordinateSpace>
	optional<Change> get_change_starting_before_position(Point target) const;

	template <typename CoordinateSpace>
	optional<Change> get_change_ending_after_position(Point target) const;

	template <typename CoordinateSpace>
	std::vector<Change> grab_changes_in_range(Point, Point, bool inclusive = false);

	template <typename CoordinateSpace>
	optional<Change> grab_change_starting_before_position(Point position);

	template <typename CoordinateSpace>
	Node *splay_node_starting_before(Point target);

	template <typename CoordinateSpace>
	Node *splay_node_starting_after(Point target, optional<Point> exclusive_lower_bound);

	template <typename CoordinateSpace>
	Node *splay_node_ending_before(Point target);

	template <typename CoordinateSpace>
	Node *splay_node_ending_after(Point target, optional<Point> exclusive_lower_bound);

	Change change_for_root_node();

	std::pair<optional<Text>, bool> compute_old_text(optional<Text> &&, Point, Point);
	uint32_t compute_old_text_size(uint32_t, Point, Point);

	void splay_node(Node *);
	void rotate_node_right(Node *, Node *, Node *);
	void rotate_node_left(Node *, Node *, Node *);
	void delete_root();
	void perform_rebalancing_rotations(uint32_t);
	Node *build_node(Node *, Node *, Point, Point, Point, Point,
									optional<Text> &&, optional<Text> &&, uint32_t old_text_size);
	void delete_node(Node **);
	void remove_noop_change();
	*/
}

#[derive(Clone)]
struct Point {
	row: usize,
	col: usize
}

impl Point {
	fn new(row: usize, col: usize) -> Self {
		Self { row, col }
	}

	fn min_point() -> Self {
		Self { row: 0, col: 0 }
	}

	fn max_point() -> Self {
		Self { row: usize::MAX, col: usize::MAX }
	}

	fn is_zero(&self) -> bool {
		self.row | self.col == 0
	}

	fn traverse(&self, traversal: &Self) -> Self {
		// superstring has checked_add function that essentially
		// performs saturating add
		if traversal.row == 0 {
			Self::new(self.row, self.col.saturating_add(traversal.col))
		} else {
			Self::new(self.row.saturating_add(traversal.row), traversal.col)
		}
	}

	fn traversal(&self, start: &Self) -> Self {
		// according to here https://en.cppreference.com/w/cpp/language/operator_arithmetic
		// cpp subtraction wraps on underflow
		if self.row == start.row {
			Self::new(0, self.col.wrapping_sub(start.col))
		} else {
			Self::new(self.row.wrapping_sub(start.row), self.col)
		}
	}
}

impl Debug for Point {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Display::fmt(self, f)
	}
}

impl Display for Point {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let Self { row, col } = self;
		write!(f, "({row}, {col})")
	}
}

impl PartialEq for Point {
	fn eq(&self, other: &Self) -> bool {
		self.row == other.row && self.col == other.col
	}
}

impl Eq for Point {}

impl PartialOrd for Point {
	fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Point {
	fn cmp(&self, other: &Point) -> Ordering {
		match self.row.cmp(&other.row) {
			Ordering::Equal => { self.col.cmp(&other.col) }
			other => { other }
		}
	}
}

#[derive(Clone)]
struct Range {
	start: Point,
	end: Point
}

impl Range {
	fn all_inclusive() -> Self {
		let start = Point::min_point();
		let end = Point::max_point();
		Self { start, end }
	}

	// impl Eq
	// Point extent() const;
}

struct Regex {
	// TODO: ?
}

impl Regex {
	/*
	pcre2_real_code_16 *code;
	Regex(pcre2_real_code_16 *);

	public:
	Regex();
	Regex(const char16_t *, uint32_t, std::u16string *error_message, bool ignore_case = false, bool unicode = false);
	Regex(const std::u16string &, std::u16string *error_message, bool ignore_case = false, bool unicode = false);
	Regex(Regex &&);
	~Regex();

	class MatchData {
		pcre2_real_match_data_16 *data;
		friend class Regex;

		public:
		MatchData(const Regex &);
		~MatchData();
	};

	struct MatchResult {
		enum {
			None,
			Error,
			Partial,
			Full,
		} type;

		size_t start_offset;
		size_t end_offset;
	};

	enum MatchOptions {
		None = 0,
		IsBeginningOfLine = 1,
		IsEndOfLine = 2,
		IsEndSearch = 4,
	};

	MatchResult match(const char16_t *data, size_t length, MatchData &, unsigned options = 0) const;
	*/
}

struct Serialiser {
	// TODO: ?
}

impl Serialiser {
	/*
	std::vector<uint8_t> &vector;

	public:
	inline Serializer(std::vector<uint8_t> &output) :
		vector(output) {};

	template <typename T>
	void append(T value) {
		for (auto i = 0u; i < sizeof(T); i++) {
			vector.push_back(value & 0xFF);
			value >>= 8;
		}
	}
	*/
}

struct Text {
	content: String,
	line_offsets: Vec<usize>
}

impl Text {
	fn empty() -> Self {
		let content = String::new();
		let line_offsets = Vec::new();
		Self { content, line_offsets }
	}

	fn with_capacity(
		content_capacity: usize,
		offsets_capacity: usize
	) -> Self {
		let content = String::with_capacity(content_capacity);
		let line_offsets = Vec::with_capacity(offsets_capacity);
		Self { content, line_offsets }
	}

	fn new_with_string(content: String) -> Self {
		let line_offsets = Self::get_line_offsets(&content);
		Self { content, line_offsets }
	}

	fn new_with_str(content: &str) -> Self {
		let line_offsets = Self::get_line_offsets(content);
		let content = content.into();
		Self { content, line_offsets }
	}

	fn get_line_offsets(content: &str) -> Vec<usize> {
		content.char_indices()
			.filter(|(_, c)| *c == '\n')
			.map(|(i, _)| i)
			.collect()
	}

	fn len(&self) -> usize {
		self.content.len()
	}

	/*
	static Point extent(const std::u16string &);

	std::u16string content;
	std::vector<uint32_t> line_offsets;
	Text(const std::u16string &&, const std::vector<uint32_t> &&);

	using const_iterator = std::u16string::const_iterator;

	Text();
	Text(const std::u16string &);
	Text(std::u16string &&);
	Text(TextSlice slice);
	Text(Deserializer &deserializer);
	template<typename Iter>
	Text(Iter begin, Iter end) : Text(std::u16string{begin, end}) {}

	static Text concat(TextSlice a, TextSlice b);
	static Text concat(TextSlice a, TextSlice b, TextSlice c);
	void splice(Point start, Point deletion_extent, TextSlice inserted_slice);

	uint16_t at(Point position) const;
	uint16_t at(uint32_t offset) const;
	const_iterator begin() const;
	const_iterator end() const;
	inline const_iterator cbegin() const { return begin(); }
	inline const_iterator cend() const { return end(); }
	ClipResult clip_position(Point) const;
	Point extent() const;
	bool empty() const;
	uint32_t offset_for_position(Point) const;
	Point position_for_offset(uint32_t, uint32_t min_row = 0, bool clip_crlf = true) const;
	uint32_t line_length_for_row(uint32_t row) const;
	void append(TextSlice);
	void assign(TextSlice);
	void serialize(Serializer &) const;
	uint32_t size() const;
	const char16_t *data() const;
	size_t digest() const;
	void clear();

	bool operator!=(const Text &) const;
	bool operator==(const Text &) const;
	*/
}

struct TextBuffer {
	// TODO: ?
}

impl TextBuffer {
	/*
	struct Layer;
	Layer *base_layer;
	Layer *top_layer;
	void squash_layers(const std::vector<Layer *> &);
	void consolidate_layers();

	public:
	static uint32_t MAX_CHUNK_SIZE_TO_COPY;

	TextBuffer();
	TextBuffer(std::u16string &&);
	TextBuffer(const std::u16string &text);
	~TextBuffer();

	uint32_t size() const;
	Point extent() const;
	optional<std::u16string> line_for_row(uint32_t row);
	void with_line_for_row(uint32_t row, const std::function<void(const char16_t *, uint32_t)> &);

	optional<uint32_t> line_length_for_row(uint32_t row);
	const uint16_t *line_ending_for_row(uint32_t row);
	ClipResult clip_position(Point);
	Point position_for_offset(uint32_t offset);
	std::u16string text();
	uint16_t character_at(Point position) const;
	std::u16string text_in_range(Range range);
	void set_text(std::u16string &&);
	void set_text(const std::u16string &);
	void set_text_in_range(Range old_range, std::u16string &&);
	void set_text_in_range(Range old_range, const std::u16string &);
	bool is_modified() const;
	bool has_astral();
	std::vector<TextSlice> chunks() const;

	void reset(Text &&);
	void flush_changes();
	void serialize_changes(Serializer &);
	bool deserialize_changes(Deserializer &);
	const Text &base_text() const;

	optional<Range> find(const Regex &, Range range = Range::all_inclusive()) const;
	std::vector<Range> find_all(const Regex &, Range range = Range::all_inclusive()) const;
	unsigned find_and_mark_all(MarkerIndex &, MarkerIndex::MarkerId, bool exclusive,
															const Regex &, Range range = Range::all_inclusive()) const;

	struct SubsequenceMatch {
		std::u16string word;
		std::vector<Point> positions;
		std::vector<uint32_t> match_indices;
		int32_t score;
		bool operator==(const SubsequenceMatch &) const;
	};

	std::vector<SubsequenceMatch> find_words_with_subsequence_in_range(const std::u16string &, const std::u16string &, Range) const;

	class Snapshot {
		friend class TextBuffer;
		TextBuffer &buffer;
		Layer &layer;
		Layer &base_layer;

		Snapshot(TextBuffer &, Layer &, Layer &);

	public:
		~Snapshot();
		void flush_preceding_changes();

		uint32_t size() const;
		Point extent() const;
		uint32_t line_length_for_row(uint32_t) const;
		std::vector<TextSlice> chunks() const;
		std::vector<TextSlice> chunks_in_range(Range) const;
		std::vector<std::pair<const char16_t *, uint32_t>> primitive_chunks() const;
		std::u16string text() const;
		std::u16string text_in_range(Range) const;
		const Text &base_text() const;
		optional<Range> find(const Regex &, Range range = Range::all_inclusive()) const;
		std::vector<Range> find_all(const Regex &, Range range = Range::all_inclusive()) const;
		std::vector<SubsequenceMatch> find_words_with_subsequence_in_range(std::u16string query, const std::u16string &extra_word_characters, Range range) const;
	};

	friend class Snapshot;
	Snapshot *create_snapshot();

	bool is_modified(const Snapshot *) const;
	Patch get_inverted_changes(const Snapshot *) const;

	size_t layer_count()  const;
	std::string get_dot_graph() const;
	*/
}

struct TextSlice {
	// TODO: ??
	// TODO: need to make sure Text is behind Box or Pin or something hh
	text: *const Text,
	start_pos: Point,
	end_pos: Point
}

impl TextSlice {
	/// unsafe because ptr? I guess?
	unsafe fn new(text: *const Text, start_pos: Point, end_pos: Point) -> Self {
		Self { text, start_pos, end_pos }
	}

	unsafe fn start_offset(&self) -> usize {
		if self.start_pos.is_zero() { return 0 }
		debug_assert!(self.start_pos.row < (*self.text).line_offsets.len());
		*(*self.text).line_offsets.as_ptr().add(self.start_pos.row) + self.start_pos.col
	}

	unsafe fn end_offset(&self) -> usize {
		if self.end_pos.is_zero() { return 0 }
		*(*self.text).line_offsets.as_ptr().add(self.end_pos.row) + self.end_pos.col
	}

	unsafe fn is_valid(&self) -> bool {
		let start_offset = self.start_offset();
		let end_offset = self.end_offset();
		if start_offset > end_offset { return false }

		// is this what you wanted, clippy?
		if self.start_pos.row + 1 < (*self.text).line_offsets.len() && start_offset >= *(*self.text).line_offsets.as_ptr().add(self.start_pos.row + 1) {
			return false
		}

		if self.end_pos.row + 1 < (*self.text).line_offsets.len() && end_offset >= *(*self.text).line_offsets.as_ptr().add(self.end_pos.row + 1) {
			return false
		}

		if end_offset > (*self.text).len() { return false }

		true
	}

	/*
	const Text *text;
	Point start_position;
	Point end_position;

	TextSlice(const Text *text, Point start_position, Point end_position);
	size_t start_offset() const;
	size_t end_offset() const;

	TextSlice();
	TextSlice(const Text &text);
	std::pair<TextSlice, TextSlice> split(Point) const;
	std::pair<TextSlice, TextSlice> split(uint32_t) const;
	TextSlice prefix(Point) const;
	TextSlice prefix(uint32_t) const;
	TextSlice suffix(Point) const;
	TextSlice slice(Range range) const;
	Point position_for_offset(uint32_t offset, uint32_t min_row = 0) const;
	Point extent() const;
	uint16_t front() const;
	uint16_t back() const;
	bool is_valid() const;

	const char16_t *data() const;
	uint32_t size() const;
	bool empty() const;

	Text::const_iterator begin() const;
	Text::const_iterator end() const;
	*/
}

// Patch text_diff(const Text &old_text, const Text &new_text);
