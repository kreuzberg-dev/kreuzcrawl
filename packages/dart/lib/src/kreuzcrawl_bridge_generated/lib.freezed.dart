// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'lib.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$AuthConfig {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String username, String password) basic,
    required TResult Function(String token) bearer,
    required TResult Function(String name, String value) header,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String username, String password)? basic,
    TResult? Function(String token)? bearer,
    TResult? Function(String name, String value)? header,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String username, String password)? basic,
    TResult Function(String token)? bearer,
    TResult Function(String name, String value)? header,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AuthConfig_Basic value) basic,
    required TResult Function(AuthConfig_Bearer value) bearer,
    required TResult Function(AuthConfig_Header value) header,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AuthConfig_Basic value)? basic,
    TResult? Function(AuthConfig_Bearer value)? bearer,
    TResult? Function(AuthConfig_Header value)? header,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AuthConfig_Basic value)? basic,
    TResult Function(AuthConfig_Bearer value)? bearer,
    TResult Function(AuthConfig_Header value)? header,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AuthConfigCopyWith<$Res> {
  factory $AuthConfigCopyWith(
          AuthConfig value, $Res Function(AuthConfig) then) =
      _$AuthConfigCopyWithImpl<$Res, AuthConfig>;
}

/// @nodoc
class _$AuthConfigCopyWithImpl<$Res, $Val extends AuthConfig>
    implements $AuthConfigCopyWith<$Res> {
  _$AuthConfigCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$AuthConfig_BasicImplCopyWith<$Res> {
  factory _$$AuthConfig_BasicImplCopyWith(_$AuthConfig_BasicImpl value,
          $Res Function(_$AuthConfig_BasicImpl) then) =
      __$$AuthConfig_BasicImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String username, String password});
}

/// @nodoc
class __$$AuthConfig_BasicImplCopyWithImpl<$Res>
    extends _$AuthConfigCopyWithImpl<$Res, _$AuthConfig_BasicImpl>
    implements _$$AuthConfig_BasicImplCopyWith<$Res> {
  __$$AuthConfig_BasicImplCopyWithImpl(_$AuthConfig_BasicImpl _value,
      $Res Function(_$AuthConfig_BasicImpl) _then)
      : super(_value, _then);

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? username = null,
    Object? password = null,
  }) {
    return _then(_$AuthConfig_BasicImpl(
      username: null == username
          ? _value.username
          : username // ignore: cast_nullable_to_non_nullable
              as String,
      password: null == password
          ? _value.password
          : password // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$AuthConfig_BasicImpl extends AuthConfig_Basic {
  const _$AuthConfig_BasicImpl({required this.username, required this.password})
      : super._();

  @override
  final String username;
  @override
  final String password;

  @override
  String toString() {
    return 'AuthConfig.basic(username: $username, password: $password)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AuthConfig_BasicImpl &&
            (identical(other.username, username) ||
                other.username == username) &&
            (identical(other.password, password) ||
                other.password == password));
  }

  @override
  int get hashCode => Object.hash(runtimeType, username, password);

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AuthConfig_BasicImplCopyWith<_$AuthConfig_BasicImpl> get copyWith =>
      __$$AuthConfig_BasicImplCopyWithImpl<_$AuthConfig_BasicImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String username, String password) basic,
    required TResult Function(String token) bearer,
    required TResult Function(String name, String value) header,
  }) {
    return basic(username, password);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String username, String password)? basic,
    TResult? Function(String token)? bearer,
    TResult? Function(String name, String value)? header,
  }) {
    return basic?.call(username, password);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String username, String password)? basic,
    TResult Function(String token)? bearer,
    TResult Function(String name, String value)? header,
    required TResult orElse(),
  }) {
    if (basic != null) {
      return basic(username, password);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AuthConfig_Basic value) basic,
    required TResult Function(AuthConfig_Bearer value) bearer,
    required TResult Function(AuthConfig_Header value) header,
  }) {
    return basic(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AuthConfig_Basic value)? basic,
    TResult? Function(AuthConfig_Bearer value)? bearer,
    TResult? Function(AuthConfig_Header value)? header,
  }) {
    return basic?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AuthConfig_Basic value)? basic,
    TResult Function(AuthConfig_Bearer value)? bearer,
    TResult Function(AuthConfig_Header value)? header,
    required TResult orElse(),
  }) {
    if (basic != null) {
      return basic(this);
    }
    return orElse();
  }
}

abstract class AuthConfig_Basic extends AuthConfig {
  const factory AuthConfig_Basic(
      {required final String username,
      required final String password}) = _$AuthConfig_BasicImpl;
  const AuthConfig_Basic._() : super._();

  String get username;
  String get password;

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AuthConfig_BasicImplCopyWith<_$AuthConfig_BasicImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AuthConfig_BearerImplCopyWith<$Res> {
  factory _$$AuthConfig_BearerImplCopyWith(_$AuthConfig_BearerImpl value,
          $Res Function(_$AuthConfig_BearerImpl) then) =
      __$$AuthConfig_BearerImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String token});
}

/// @nodoc
class __$$AuthConfig_BearerImplCopyWithImpl<$Res>
    extends _$AuthConfigCopyWithImpl<$Res, _$AuthConfig_BearerImpl>
    implements _$$AuthConfig_BearerImplCopyWith<$Res> {
  __$$AuthConfig_BearerImplCopyWithImpl(_$AuthConfig_BearerImpl _value,
      $Res Function(_$AuthConfig_BearerImpl) _then)
      : super(_value, _then);

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? token = null,
  }) {
    return _then(_$AuthConfig_BearerImpl(
      token: null == token
          ? _value.token
          : token // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$AuthConfig_BearerImpl extends AuthConfig_Bearer {
  const _$AuthConfig_BearerImpl({required this.token}) : super._();

  @override
  final String token;

  @override
  String toString() {
    return 'AuthConfig.bearer(token: $token)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AuthConfig_BearerImpl &&
            (identical(other.token, token) || other.token == token));
  }

  @override
  int get hashCode => Object.hash(runtimeType, token);

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AuthConfig_BearerImplCopyWith<_$AuthConfig_BearerImpl> get copyWith =>
      __$$AuthConfig_BearerImplCopyWithImpl<_$AuthConfig_BearerImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String username, String password) basic,
    required TResult Function(String token) bearer,
    required TResult Function(String name, String value) header,
  }) {
    return bearer(token);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String username, String password)? basic,
    TResult? Function(String token)? bearer,
    TResult? Function(String name, String value)? header,
  }) {
    return bearer?.call(token);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String username, String password)? basic,
    TResult Function(String token)? bearer,
    TResult Function(String name, String value)? header,
    required TResult orElse(),
  }) {
    if (bearer != null) {
      return bearer(token);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AuthConfig_Basic value) basic,
    required TResult Function(AuthConfig_Bearer value) bearer,
    required TResult Function(AuthConfig_Header value) header,
  }) {
    return bearer(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AuthConfig_Basic value)? basic,
    TResult? Function(AuthConfig_Bearer value)? bearer,
    TResult? Function(AuthConfig_Header value)? header,
  }) {
    return bearer?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AuthConfig_Basic value)? basic,
    TResult Function(AuthConfig_Bearer value)? bearer,
    TResult Function(AuthConfig_Header value)? header,
    required TResult orElse(),
  }) {
    if (bearer != null) {
      return bearer(this);
    }
    return orElse();
  }
}

abstract class AuthConfig_Bearer extends AuthConfig {
  const factory AuthConfig_Bearer({required final String token}) =
      _$AuthConfig_BearerImpl;
  const AuthConfig_Bearer._() : super._();

  String get token;

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AuthConfig_BearerImplCopyWith<_$AuthConfig_BearerImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AuthConfig_HeaderImplCopyWith<$Res> {
  factory _$$AuthConfig_HeaderImplCopyWith(_$AuthConfig_HeaderImpl value,
          $Res Function(_$AuthConfig_HeaderImpl) then) =
      __$$AuthConfig_HeaderImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String name, String value});
}

/// @nodoc
class __$$AuthConfig_HeaderImplCopyWithImpl<$Res>
    extends _$AuthConfigCopyWithImpl<$Res, _$AuthConfig_HeaderImpl>
    implements _$$AuthConfig_HeaderImplCopyWith<$Res> {
  __$$AuthConfig_HeaderImplCopyWithImpl(_$AuthConfig_HeaderImpl _value,
      $Res Function(_$AuthConfig_HeaderImpl) _then)
      : super(_value, _then);

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? value = null,
  }) {
    return _then(_$AuthConfig_HeaderImpl(
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$AuthConfig_HeaderImpl extends AuthConfig_Header {
  const _$AuthConfig_HeaderImpl({required this.name, required this.value})
      : super._();

  @override
  final String name;
  @override
  final String value;

  @override
  String toString() {
    return 'AuthConfig.header(name: $name, value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AuthConfig_HeaderImpl &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, name, value);

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AuthConfig_HeaderImplCopyWith<_$AuthConfig_HeaderImpl> get copyWith =>
      __$$AuthConfig_HeaderImplCopyWithImpl<_$AuthConfig_HeaderImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String username, String password) basic,
    required TResult Function(String token) bearer,
    required TResult Function(String name, String value) header,
  }) {
    return header(name, value);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String username, String password)? basic,
    TResult? Function(String token)? bearer,
    TResult? Function(String name, String value)? header,
  }) {
    return header?.call(name, value);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String username, String password)? basic,
    TResult Function(String token)? bearer,
    TResult Function(String name, String value)? header,
    required TResult orElse(),
  }) {
    if (header != null) {
      return header(name, value);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AuthConfig_Basic value) basic,
    required TResult Function(AuthConfig_Bearer value) bearer,
    required TResult Function(AuthConfig_Header value) header,
  }) {
    return header(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AuthConfig_Basic value)? basic,
    TResult? Function(AuthConfig_Bearer value)? bearer,
    TResult? Function(AuthConfig_Header value)? header,
  }) {
    return header?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AuthConfig_Basic value)? basic,
    TResult Function(AuthConfig_Bearer value)? bearer,
    TResult Function(AuthConfig_Header value)? header,
    required TResult orElse(),
  }) {
    if (header != null) {
      return header(this);
    }
    return orElse();
  }
}

abstract class AuthConfig_Header extends AuthConfig {
  const factory AuthConfig_Header(
      {required final String name,
      required final String value}) = _$AuthConfig_HeaderImpl;
  const AuthConfig_Header._() : super._();

  String get name;
  String get value;

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AuthConfig_HeaderImplCopyWith<_$AuthConfig_HeaderImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
